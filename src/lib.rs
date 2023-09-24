#![no_std]

/// Minimum allowed DSHOT throttle value
pub const THROTTLE_MIN: u16 = 48;
/// Maximum allowed DSHOT throttle value
pub const THROTTLE_MAX: u16 = 2047;

#[derive(Copy, Clone)]
#[repr(u16)]
/// Dshot commands, see `send_command()`
pub enum DshotCmdT {
    /// Currently not implemented
    DigitalCmdMotorStop,
    /// Wait at least length of beep (260ms) before next command
    DigitalCmdBeep1,
    /// Wait at least length of beep (260ms) before next command
    DigitalCmdBeep2,
    /// Wait at least length of beep (280ms) before next command
    DigitalCmdBeep3,
    /// Wait at least length of beep (280ms) before next command
    DigitalCmdBeep4,
    /// Wait at least length of beep (1020ms) before next command
    DigitalCmdBeep5,
    /// Wait at least 12ms before next command
    DigitalCmdEscInfo,
    /// Need 6x, no wait required
    DigitalCmdSpinDirection1,
    /// Need 6x, no wait required
    DigitalCmdSpinDirection2,
    /// Need 6x, no wait required
    DigitalCmd3dModeOff,
    /// Need 6x, no wait required
    DigitalCmd3dModeOn,
    /// Currently not implemented
    DigitalCmdSettingsRequest,
    /// Need 6x, wait at least 35ms before next command
    DigitalCmdSaveSettings,
    /// Need 6x, no wait required
    DigitalCmdSpinDirectionNormal = 20,
    /// Need 6x, no wait required
    DigitalCmdSpinDirectionReversed,
    /// No wait required
    DigitalCmdLed0On,
    /// No wait required
    DigitalCmdLed1On,
    /// No wait required
    DigitalCmdLed2On,
    /// No wait required
    DigitalCmdLed3On,
    /// No wait required
    DigitalCmdLed0Off,
    /// No wait required
    DigitalCmdLed1Off,
    /// No wait required
    DigitalCmdLed2Off,
    /// No wait required
    DigitalCmdLed3Off,
}

#[derive(Debug)]
/// Errors relevant to DSHOT
pub enum DshotError {
    /// The given throttle value is smaller or larger than limits
    InvalidThrottleValue(u16),
}

/// Calculate the CRC checksum for a packet
fn calc_checksum(command: u16, telemetry: bool, inverted: bool) -> u16 {
    // Concatenate throttle value with telemetry request
    let packet = (command << 1) | (telemetry as u16);

    // Calculate and return checksum
    if inverted {
        (!(packet ^ (packet >> 4) ^ (packet >> 8))) & 0x0F
    } else {
        (packet ^ (packet >> 4) ^ (packet >> 8)) & 0x0F
    }
}

/// Calculate the DSHOT frame for any 11-bit message.
/// For non-throttle commands, the telemetry flag has to be set
fn any_message(message: u16, telemetry: bool, inverted: bool) -> u16 {
    // Get checksum
    let checksum = calc_checksum(message, telemetry, inverted);

    // Assemble packet
    message << 5 | (telemetry as u16) << 4 | checksum
}

/// Calculate the DSHOT frame for a throttle value between 48 and 2047
pub fn throttle(throttle: u16, telemetry: bool, inverted: bool) -> Result<u16, DshotError> {
    // Early return with error if throttle is out of range
    if throttle < THROTTLE_MIN || throttle > THROTTLE_MAX {
        return Err(DshotError::InvalidThrottleValue(throttle));
    }

    Ok(any_message(throttle, telemetry, inverted))
}

/// Calculate the DSHOT frame for any valid command
pub fn command(command: DshotCmdT, inverted: bool) -> u16 {
    // Telemetry bit must be set in command frames
    let telemetry = true;

    any_message(command as u16, telemetry, inverted)
}

/// Calculate the DSHOT frame for reversing the motor spin direction
pub fn reverse(reverse: bool, inverted: bool) -> u16 {
    command(match reverse {
        true => DshotCmdT::DigitalCmdSpinDirectionReversed,
        false => DshotCmdT::DigitalCmdSpinDirectionNormal,
    }, inverted)
}

/// Calculate the DSHOT frame where throttle is clamped to be between 48 and 2047
pub fn throttle_clamp(throttle: u16, telemetry: bool, inverted: bool) -> u16 {
    any_message(throttle.clamp(THROTTLE_MIN, THROTTLE_MAX), telemetry, inverted)
}

/// Calculate the DSHOT frame for a minimum throttle value
pub fn throttle_minimum(telemetry: bool, inverted: bool) -> u16 {
    any_message(THROTTLE_MIN, telemetry, inverted)
}

/// almost entirely sourced from https://github.com/betaflight/betaflight/blob/master/src/main/drivers/dshot.c#L140-L155
pub fn e_rpm_from_telemetry_value(value: u16) -> u32 {
    // eRPM range
    if value == 0x0fff {
        return 0;
    }

    let mut value: u32 = value.into();

    // Convert value to 16 bit from the GCR telemetry format (eeem mmmm mmmm)
    value = (value & 0x01ff) << ((value & 0xfe00) >> 9);

    // Convert period to erpm * 100
    (1000000 * 60 / 100 + value / 2) / value
}
