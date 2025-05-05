CC = gcc
PROGRAM_NAME = shoco
RUST_TARGET = libshoco.a
C_TARGET = $(PROGRAM_NAME)_test

all: c_test rust_test

c_test:
	$(CC) -o $(C_TARGET) $(PROGRAM_NAME).c $(C_TARGET).c
	@echo "Built C implementation tests"

rust_test:
	cargo build --release
	gcc -o $(C_TARGET) $(C_TARGET).c ./target/release/$(RUST_TARGET) -lpthread -ldl
	@echo "Built Rust implementation tests"

run_c:
	./$(C_TARGET)

run_rust:
	./$(C_TARGET)

clean:
	cargo clean
	rm -f $(C_TARGET)