This crate provides a handful of utility types for writing abstractions for interfacing with register based devices. Most commonly, this would be utilized when writing drivers for external peripherals within an embedded environment. As such, some utility functions are provided for reading and writing registers on devices across I2C or SPI buses.

This crate provides two core traits:
- `Register` for types that represent a value stored within an addressable register
- `Command` for types that represent an invokable command with parameters and response

### Register Implementation

There are two ways to implement registers: using the `register` attribute macro for a simplified approach, or manually implementing the traits for more control.

#### Using the Register Attribute Macro

The simplest way to define a register is using the `register` attribute macro along with the derive macros:

```rust
use regiface::{register, ReadableRegister, WritableRegister, FromByteArray, ToByteArray};

// Define a register with an ID of 42u8
#[register(42u8)]
#[derive(ReadableRegister, WritableRegister, Debug)]
struct MyRegister {
    value: u8
}

// Just implement the conversion traits
impl FromByteArray for MyRegister {
    type Error = core::convert::Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            value: bytes[0]
        })
    }
}

impl ToByteArray for MyRegister {
    type Error = core::convert::Infallible;
    type Array = [u8; 1];

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok([self.value])
    }
}
```

#### Manual Implementation

For more control, you can manually implement the required traits:

```rust
use regiface::{Register, ReadableRegister, FromByteArray};

// A type we will use to represent some fictional register 
struct MyRegister {    
    value: u8
}

// Implement the Register trait, and specify it has an ID of 42u8
impl Register for MyRegister {    
    type IdType = u8;

    fn id() -> Self::IdType {
        42    
    }
}

// Implement the FromByteArray trait, and specify it can be converted from a 1-byte array 
impl FromByteArray for MyRegister {
    type Error = core::convert::Infallible;
    type Array = [u8; 1];

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            value: bytes[0]        
        })    
    }
}

// Indicate this is a readable register!
impl ReadableRegister for MyRegister {}
```

### Complete Example

Here's a complete example showing how to use registers and commands with both I2C and SPI devices:

```rust
use regiface::{
    register, Command, FromByteArray, ReadableRegister, ToByteArray, WritableRegister,
    NoParameters,
};

// Temperature register that can be read
#[register(0x00)]
#[derive(ReadableRegister)]
struct Temperature {
    celsius: f32,
}

impl FromByteArray for Temperature {
    type Array = [u8; 4];
    type Error = &'static str;

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            celsius: f32::from_be_bytes(bytes),
        })
    }
}

// Configuration register that can be written
#[register(0x01)]
#[derive(WritableRegister)]
struct Configuration {
    sample_rate: u16,
    enabled: bool,
}

impl ToByteArray for Configuration {
    type Array = [u8; 3];
    type Error = &'static str;

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        let mut bytes = [0u8; 3];
        bytes[0..2].copy_from_slice(&self.sample_rate.to_be_bytes());
        bytes[2] = self.enabled as u8;
        Ok(bytes)
    }
}

// Command to perform calibration
struct Calibrate;

#[derive(Default)]
struct CalibrationParams {
    reference_temp: f32,
}

impl ToByteArray for CalibrationParams {
    type Array = [u8; 4];
    type Error = &'static str;

    fn to_bytes(self) -> Result<Self::Array, Self::Error> {
        Ok(self.reference_temp.to_be_bytes())
    }
}

struct CalibrationResponse {
    offset: f32,
}

impl FromByteArray for CalibrationResponse {
    type Array = [u8; 4];
    type Error = &'static str;

    fn from_bytes(bytes: Self::Array) -> Result<Self, Self::Error> {
        Ok(Self {
            offset: f32::from_be_bytes(bytes),
        })
    }
}

impl Command for Calibrate {
    type CommandParameters = CalibrationParams;
    type ResponseParameters = CalibrationResponse;

    fn id() -> regiface::id::RegisterId {
        0xF0.into()
    }

    fn invoking_parameters(self) -> Self::CommandParameters {
        CalibrationParams {
            reference_temp: 25.0,
        }
    }
}

// Using with I2C (async)
use embedded_hal_async::i2c::I2c;

async fn use_i2c_device<D: I2c<u8>>(i2c: &mut D) {
    const DEVICE_ADDR: u8 = 0x48;

    // Read temperature (type inference)
    let temp: Temperature = regiface::i2c::r#async::read_register(i2c, DEVICE_ADDR)
        .await
        .unwrap();
    println!("Temperature: {}°C", temp.celsius);

    // Write configuration
    let config = Configuration {
        sample_rate: 100,
        enabled: true,
    };
    regiface::i2c::r#async::write_register(i2c, DEVICE_ADDR, config)
        .await
        .unwrap();

    // Execute calibration command
    let result: CalibrationResponse = regiface::i2c::r#async::invoke_command(
        i2c,
        DEVICE_ADDR,
        Calibrate,
    )
    .await
    .unwrap();
    println!("Calibration offset: {}", result.offset);
}

// Using with SPI (blocking)
use embedded_hal::spi::SpiDevice;

fn use_spi_device<D: SpiDevice>(spi: &mut D) {
    // Read temperature (type inference)
    let temp: Temperature = regiface::spi::blocking::read_register(spi).unwrap();
    println!("Temperature: {}°C", temp.celsius);

    // Write configuration
    let config = Configuration {
        sample_rate: 100,
        enabled: true,
    };
    regiface::spi::blocking::write_register(spi, config).unwrap();

    // Execute calibration command
    let result: CalibrationResponse =
        regiface::spi::blocking::invoke_command(spi, Calibrate).unwrap();
    println!("Calibration offset: {}", result.offset);
}
```

### Readable Registers

A register in which values can be retrieved from, or read from, is represented as any type that implements the `ReadableRegister` trait. This trait is very little more than just a marker trait, but it represents a type that is both a `Register` and that can be created from a byte array through the `FromByteArray` trait. The bulk of the work in writing a type that can be read from a register will be in implementing the `FromByteArray` trait.

A type that implements the `ReadableRegister` trait can then be used with provided utility methods such as those provided by the `i2c` or `spi` modules.

### Writable Registers

A register in which values can be written to is represented as any type that implements the `WritableRegister` trait. This trait is very little more than just a marker trait, but it represents a type that is both a `Register` and that can be serialized into a byte array through the `ToByteArray` trait. The bulk of the work in writing a type that can be written to a register will be in implementing the `ToByteArray` trait.

A type that implements the `WritableRegister` trait can then be used with provided utility methods such as those provided by the `i2c` or `spi` modules.

### Commands

A command represents an invokable action with optional parameters and response. Commands are implemented using the `Command` trait, which specifies both the command parameters and expected response type. For commands or responses without parameters, the `NoParameters` type can be used.

The command's parameters must implement `ToByteArray` for serialization, and its response type must implement `FromByteArray` for deserialization. The command itself specifies an ID that uniquely identifies it to the device.
