build:
	@xargo build --release

flash: build
	arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/release/rusty-print target/hex
	teensy-loader-cli -w -s -mmcu=mk20dx128 target/hex

clean:
	xargo clean
	git clean -Xf

