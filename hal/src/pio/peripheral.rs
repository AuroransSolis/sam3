//! PIO peripheral configuration
//!
//!

use core::marker::PhantomData;

pub trait MultiDriverCfg {}
pub trait PioControl {}
pub trait PeripheralSelectCfg {}
pub trait OutputCfg {}
pub trait OutputWriteCfg {}
pub trait OutputSyncWriteCfg {}

/// Disable multidrive on this PIO line.
pub struct MultiDriverDisabled<Line: PioControl> {
    _line: PhantomData<Line>,
}

impl<Line: PioControl> MultiDriverCfg for MultiDriverDisabled<Line> {}

/// Enable multidrive on this PIO line. When configured in this state, drivers should only drive the
/// line low. Additionally, a pull-up resistor is generally required to ensure that the line can
/// achieve a high level.
pub struct MultiDriverEnabled<Psel: PioControl, Sync: OutputSyncWriteCfg> {
    _psel: PhantomData<Psel>,
    _sync: PhantomData<Sync>,
}

impl<Line: PioControl, Sync: OutputSyncWriteCfg> MultiDriverCfg for MultiDriverEnabled<Line, Sync> {}

/// Allow the peripheral to control this I/O line.
pub struct PeripheralControlled<Psel: PeripheralSelectCfg> {
    _psel: PhantomData<Psel>,
}

impl<Psel: PeripheralSelectCfg> PioControl for PeripheralControlled<Psel> {}

/// Allow the PIO controller to control this I/O.
pub struct PioControlled<Otpt: OutputCfg> {
    _otpt: PhantomData<Otpt>,
}

impl<Otpt: OutputCfg> PioControl for PioControlled<Otpt> {}

/// Allow output from peripheral A to drive the I/O line.
pub struct PeripheralA;

impl PeripheralSelectCfg for PeripheralA {}

/// Allow output from peripheral B to drive the I/O line.
pub struct PeripheralB;

impl PeripheralSelectCfg for PeripheralB {}

/// Disable PIO control of the output.
pub struct OutputDisabled;

impl OutputCfg for OutputDisabled {}

/// Enable PIO control of the output.
pub struct OutputEnabled<Outw: OutputWriteCfg, Sync: OutputSyncWriteCfg> {
    _outw: PhantomData<Outw>,
    _sync: PhantomData<Sync>,
}

impl<Outw: OutputWriteCfg, Sync: OutputSyncWriteCfg> OutputCfg for OutputEnabled<Outw, Sync> {}

/// Drive the output of an I/O line high.
pub struct SetOutput;

impl OutputWriteCfg for SetOutput {}

/// Pull the output of an I/O line low.
pub struct ClearOutput;

impl OutputWriteCfg for ClearOutput {}

/// Allow the output of this PIO line to be set or cleared synchronously with other PIO lines
/// by writing to `PIO_ODSR`.
pub struct SyncOutputEnabled;

impl OutputSyncWriteCfg for SyncOutputEnabled {}

/// Disable synchronously setting the output of this PIO line with others.
pub struct SyncOutputDisabled;

impl OutputSyncWriteCfg for SyncOutputDisabled {}
