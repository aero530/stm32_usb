# STM32 Rust USB Test

Test application to see if USB works on STM32 in Rust...spoilers...not so much.

## STM32F303

### config.toml

```toml
[target.thumbv7em-none-eabi]

# use the Tlink.x scrip from the cortex-m-rt crate
rustflags = [
  "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv7em-none-eabi"
```

### Cargo.toml

[F3xx HAL](https://github.com/stm32-rs/stm32f3xx-hal)

```toml
[dependencies.stm32f3xx-hal]
version = "0.9.1"
features = ["stm32f303xe", "rt"]
```

### memory.x

```c
/* memory.x - Linker script for the STM32F303ZET6 */
MEMORY
{
  /* Flash memory begins at 0x80000000 and has a size of 512KB*/
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  /* RAM begins at 0x20000000 and has a size of 64kB*/
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}
```

### Compile

```console
> cargo build --target thumbv7em-none-eabi --release
```

### Flash

```console
> cargo flash --target thumbv7em-none-eabi --chip STM32F303ZETx --release
```
