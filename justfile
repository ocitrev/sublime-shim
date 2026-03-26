build:
    cargo build

install: install-subl install-smerge

install-subl:
    cargo install --path ./subl

install-smerge:
    cargo install --path ./smerge
