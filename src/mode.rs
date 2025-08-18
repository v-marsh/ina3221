/// Represents the operating mode of the INA3221
#[derive(Debug)]
pub enum OperatingMode {
    /// Power save mode, no measurements are performed
    PowerDown = 0x00,
    /// Shunt and bus voltage measurements are performed once
    Triggered = 0x03,
    /// Shunt and bus voltage measurements are performed continuously
    Continuous = 0x07,
}

#[allow(dead_code)]
#[derive(Debug,Clone,Copy)]
pub enum AveragingMode{
    Samples1 = 0b000,
    Samples4 = 0b001,
    Samples16 = 0b010,
    Samples64 = 0b011,
    Samples128 = 0b100,
    Samples256 = 0b101,
    Samples512 = 0b110,
    Samples1024 = 0b111,
}

#[allow(dead_code)]
#[derive(Debug,Clone,Copy)]
pub enum ConversionTime{
    Us140 = 0b000,
    Us204 = 0b001,
    Us332 = 0b010,
    Us588 = 0b011,
    Us1100 = 0b100,
    Us2116 = 0b101,
    Us4156 = 0b110,
    Us8244 = 0b111,
}
