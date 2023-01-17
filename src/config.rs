use crate::error::SensorError;

pub(crate) trait Bitfield {
    const BITMASK: u8;

    /// Bit value of a discriminant, shifted to the correct position if
    /// necessary
    fn bits(self) -> u8;
}

/// I²C slave addresses, determined by the logic level of pin `AP_AD0`
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Address {
    /// `AP_AD0` pin == 0
    Primary = 0x68,
    /// `AP_AD0` pin == 1
    Secondary = 0x69,
}

/// Configurable ranges of the Accelerometer
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccelRange {
    /// ±2G
    G2 = 3,
    /// ±4G
    G4 = 2,
    /// ±8G
    G8 = 1,
    /// ±16G
    G16 = 0,
}

impl AccelRange {
    /// Sensitivity scale factor
    pub fn scale_factor(&self) -> f32 {
        use AccelRange::*;

        // Values taken from Table 2 of the data sheet
        match self {
            G2 => 16_384.0,
            G4 => 8_192.0,
            G8 => 4_096.0,
            G16 => 2_048.0,
        }
    }
}

impl Bitfield for AccelRange {
    const BITMASK: u8 = 0b0110_0000;

    fn bits(self) -> u8 {
        // `ACCEL_UI_FS_SEL` occupies bits 6:5 in the register
        (self as u8) << 5
    }
}

impl Default for AccelRange {
    fn default() -> Self {
        Self::G16
    }
}

impl TryFrom<u8> for AccelRange {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AccelRange::*;

        match value {
            0 => Ok(G16),
            1 => Ok(G8),
            2 => Ok(G4),
            3 => Ok(G2),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Configurable ranges of the Gyroscope
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GyroRange {
    /// ±250 deg/sec
    Deg250 = 3,
    /// ±500 deg/sec
    Deg500 = 2,
    /// ±1000 deg/sec
    Deg1000 = 1,
    /// ±2000 deg/sec
    Deg2000 = 0,
}

impl GyroRange {
    /// Sensitivity scale factor
    pub fn scale_factor(&self) -> f32 {
        use GyroRange::*;

        // Values taken from Table 1 of the data sheet
        match self {
            Deg250 => 131.0,
            Deg500 => 65.5,
            Deg1000 => 32.8,
            Deg2000 => 16.4,
        }
    }
}

impl Bitfield for GyroRange {
    const BITMASK: u8 = 0b0110_0000;

    fn bits(self) -> u8 {
        // `GYRO_UI_FS_SEL` occupies bits 6:5 in the register
        (self as u8) << 5
    }
}

impl Default for GyroRange {
    fn default() -> Self {
        Self::Deg2000
    }
}

impl TryFrom<u8> for GyroRange {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GyroRange::*;

        match value {
            0 => Ok(Deg2000),
            1 => Ok(Deg1000),
            2 => Ok(Deg500),
            3 => Ok(Deg250),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Configurable power modes of the IMU
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    /// Gyroscope: OFF, Accelerometer: OFF
    Sleep = 0b00000,
    /// Gyroscope: DRIVE ON, Accelerometer: OFF
    Standby = 0b00100,
    /// Gyroscope: OFF, Accelerometer: DUTY-CYCLED
    AccelLowPower = 0b00010,
    /// Gyroscope: OFF, Accelerometer: ON
    AccelLowNoise = 0b00011,
    /// Gyroscope: ON, Accelerometer: OFF
    GyroLowNoise = 0b01100,
    /// Gyroscope: ON, Accelerometer: ON
    SixAxisLowNoise = 0b01111,
    /// Idle mode RC oscilator is powered on Accel and Gyro OFF
    Idle = 0b10000,
}

impl Bitfield for PowerMode {
    const BITMASK: u8 = 0b0001_1111;

    fn bits(self) -> u8 {
        // `GYRO_MODE` occupies bits 3:2 in the register
        // `ACCEL_MODE` occupies bits 1:0 in the register
        self as u8
    }
}

impl Default for PowerMode {
    fn default() -> Self {
        PowerMode::Sleep
    }
}

impl TryFrom<u8> for PowerMode {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use PowerMode::*;

        match value {
            0b0000 => Ok(Sleep),
            0b0100 => Ok(Standby),
            0b0010 => Ok(AccelLowPower),
            0b0011 => Ok(AccelLowNoise),
            0b1100 => Ok(GyroLowNoise),
            0b1111 => Ok(SixAxisLowNoise),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Accelerometer ODR selection values
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccelOdr {
    /// 1.6 kHz (LN mode)
    Hz1600 = 0b0101,
    /// 800 Hz (LN mode
    Hz800 = 0b0110,
    /// 400 Hz (LP or LN mode)
    Hz400 = 0b0111,
    /// 200 Hz (LP or LN mode)
    Hz200 = 0b1000,
    /// 100 Hz (LP or LN mode)
    Hz100 = 0b1001,
    /// 50 Hz (LP or LN mode)
    Hz50 = 0b1010,
    /// 25 Hz (LP or LN mode)
    Hz25 = 0b1011,
    /// 12.5 Hz (LP or LN mode)
    Hz12_5 = 0b1100,
    /// 6.25 Hz (LP mode)
    Hz6_25 = 0b1101,
    /// 3.125 Hz (LP mode)
    Hz3_125 = 0b1110,
    /// 1.5625 Hz (LP mode
    Hz1_5625 = 0b1111,
}

impl AccelOdr {
    pub fn as_f32(self) -> f32 {
        use AccelOdr::*;

        match self {
            Hz1600 => 1600.0,
            Hz800 => 800.0,
            Hz400 => 400.0,
            Hz200 => 200.0,
            Hz100 => 100.0,
            Hz50 => 50.0,
            Hz25 => 25.0,
            Hz12_5 => 12.5,
            Hz6_25 => 6.25,
            Hz3_125 => 3.125,
            Hz1_5625 => 1.5625,
        }
    }
}

impl Bitfield for AccelOdr {
    const BITMASK: u8 = 0b0000_1111;

    fn bits(self) -> u8 {
        // `ACCEL_ODR` occupies bits 3:0 in the register
        self as u8
    }
}

impl Default for AccelOdr {
    fn default() -> Self {
        Self::Hz800
    }
}

impl TryFrom<u8> for AccelOdr {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AccelOdr::*;

        match value {
            0b0101 => Ok(Hz1600),
            0b0110 => Ok(Hz800),
            0b0111 => Ok(Hz400),
            0b1000 => Ok(Hz200),
            0b1001 => Ok(Hz100),
            0b1010 => Ok(Hz50),
            0b1011 => Ok(Hz25),
            0b1100 => Ok(Hz12_5),
            0b1101 => Ok(Hz6_25),
            0b1110 => Ok(Hz3_125),
            0b1111 => Ok(Hz1_5625),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Gyroscope ODR selection values
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GyroOdr {
    /// 1.6k Hz
    Hz1600 = 0b0101,
    /// 800 Hz
    Hz800 = 0b0110,
    /// 400 Hz
    Hz400 = 0b0111,
    /// 200 Hz
    Hz200 = 0b1000,
    /// 100 Hz
    Hz100 = 0b1001,
    /// 50 Hz
    Hz50 = 0b1010,
    /// 25 Hz
    Hz25 = 0b1011,
    /// 12.5 Hz
    Hz12_5 = 0b1100,
}

impl GyroOdr {
    pub fn as_f32(self) -> f32 {
        use GyroOdr::*;

        match self {
            Hz1600 => 1600.0,
            Hz800 => 800.0,
            Hz400 => 400.0,
            Hz200 => 200.0,
            Hz100 => 100.0,
            Hz50 => 50.0,
            Hz25 => 25.0,
            Hz12_5 => 12.5,
        }
    }
}

impl Bitfield for GyroOdr {
    const BITMASK: u8 = 0b0000_1111;

    fn bits(self) -> u8 {
        // `GYRO_ODR` occupies bits 3:0 in the register
        self as u8
    }
}

impl Default for GyroOdr {
    fn default() -> Self {
        Self::Hz800
    }
}

impl TryFrom<u8> for GyroOdr {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GyroOdr::*;

        match value {
            0b0101 => Ok(Hz1600),
            0b0110 => Ok(Hz800),
            0b0111 => Ok(Hz400),
            0b1000 => Ok(Hz200),
            0b1001 => Ok(Hz100),
            0b1010 => Ok(Hz50),
            0b1011 => Ok(Hz25),
            0b1100 => Ok(Hz12_5),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Gyroscope Filter Bandwith selection values
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GyroBw {
    /// BW filter bypassed
    Hz10000 = 0b000,
    /// 180 Hz
    Hz180 = 0b001,
    /// 121 Hz
    Hz121 = 0b010,
    /// 73 Hz
    Hz73 = 0b011,
    /// 53 Hz
    Hz53 = 0b100,
    /// 34 Hz
    Hz34 = 0b101,
    /// 25 Hz
    Hz25 = 0b110,
    /// 16 Hz
    Hz16 = 0b111,
}

impl GyroBw {
    pub fn as_f32(self) -> f32 {
        use GyroBw::*;

        match self {
            Hz10000 => 10000.0, // filter is bypassed
            Hz180 => 180.0,
            Hz121 => 121.0,
            Hz73 => 73.0,
            Hz53 => 53.0,
            Hz34 => 34.0,
            Hz25 => 25.0,
            Hz16 => 16.0,
        }
    }
}

impl Default for GyroBw {
    fn default() -> Self {
        Self::Hz180
    }
}

impl Bitfield for GyroBw {
    const BITMASK: u8 = 0b0000_0111;

    fn bits(self) -> u8 {
        // `GYRO_UI_FILT_BW` occupies bits 2:0 in the register
        self as u8
    }
}

impl TryFrom<u8> for GyroBw {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use GyroBw::*;

        match value {
            0b000 => Ok(Hz10000), // filter is bypassed
            0b001 => Ok(Hz180),
            0b010 => Ok(Hz121),
            0b011 => Ok(Hz73),
            0b100 => Ok(Hz53),
            0b101 => Ok(Hz34),
            0b110 => Ok(Hz25),
            0b111 => Ok(Hz16),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}

/// Accelareration Filter Bandwith selection values
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccelBw {
    /// BW filter bypassed
    Hz10000 = 0b000,
    /// 180 Hz
    Hz180 = 0b001,
    /// 121 Hz
    Hz121 = 0b010,
    /// 73 Hz
    Hz73 = 0b011,
    /// 53 Hz
    Hz53 = 0b100,
    /// 34 Hz
    Hz34 = 0b101,
    /// 25 Hz
    Hz25 = 0b110,
    /// 16 Hz
    Hz16 = 0b111,
}

impl AccelBw {
    pub fn as_f32(self) -> f32 {
        use AccelBw::*;

        match self {
            Hz10000 => 10000.0, // filter is bypassed
            Hz180 => 180.0,
            Hz121 => 121.0,
            Hz73 => 73.0,
            Hz53 => 53.0,
            Hz34 => 34.0,
            Hz25 => 25.0,
            Hz16 => 16.0,
        }
    }
}

impl Default for AccelBw {
    fn default() -> Self {
        Self::Hz180
    }
}

impl Bitfield for AccelBw {
    const BITMASK: u8 = 0b0000_0111;

    fn bits(self) -> u8 {
        // `ACCEL_UI_FILT_BW` occupies bits 2:0 in the register
        self as u8
    }
}

impl TryFrom<u8> for AccelBw {
    type Error = SensorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AccelBw::*;

        match value {
            0b000 => Ok(Hz10000), // filter is bypassed
            0b001 => Ok(Hz180),
            0b010 => Ok(Hz121),
            0b011 => Ok(Hz73),
            0b100 => Ok(Hz53),
            0b101 => Ok(Hz34),
            0b110 => Ok(Hz25),
            0b111 => Ok(Hz16),
            _ => Err(SensorError::InvalidDiscriminant),
        }
    }
}
