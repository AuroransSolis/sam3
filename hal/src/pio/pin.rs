use crate::pio::{
    filter::InputFilterCfg, interrupt::InterruptCfg, peripheral::MultiDriverCfg, IsPio,
};
use core::marker::PhantomData;
use paste::paste;

#[allow(clippy::module_name_repetitions)]
pub trait PinId {
    type Controller: IsPio;
    const MASK: u32;
}

pub struct Pin<Pio, Pid, Mdvr, Pupr, Irpt, Filt>
where
    Pio: IsPio,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Pupr: PullupResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    _pio: PhantomData<Pio>,
    _pid: PhantomData<Pid>,
    _mdvr: PhantomData<Mdvr>,
    _pupr: PhantomData<Pupr>,
    _irpt: PhantomData<Irpt>,
    _filt: PhantomData<Filt>,
}

impl<Pio, Pid, Mdvr, Pupr, Irpt, Filt> Pin<Pio, Pid, Mdvr, Pupr, Irpt, Filt>
where
    Pio: IsPio,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg,
    Pupr: PullupResistorCfg,
    Irpt: InterruptCfg,
    Filt: InputFilterCfg,
{
    pub(crate) unsafe fn new() -> Self {
        Pin {
            _pio: PhantomData,
            _pid: PhantomData,
            _mdvr: PhantomData,
            _pupr: PhantomData,
            _irpt: PhantomData,
            _filt: PhantomData,
        }
    }
}

pub trait Configured {}

#[rustfmt::skip]
impl<Pio, Pid, Mdvr, Pupr, Irpt, Filt> Configured for Pin<Pio, Pid, Mdvr, Pupr, Irpt, Filt>
where
    Pio: IsPio,
    Pid: PinId<Controller = Pio>,
    Mdvr: MultiDriverCfg + Configured,
    Pupr: PullupResistorCfg + Configured,
    Irpt: InterruptCfg + Configured,
    Filt: InputFilterCfg + Configured,
{}

pub struct Unconfigured;

pub trait PullupResistorCfg {}

/// Enable the pull-up resistor on an I/O line.
pub struct PullupEnabled;

impl PullupResistorCfg for PullupEnabled {}

/// Disable the pull-up resistor on an I/O line.
pub struct PullupDisabled;

impl PullupResistorCfg for PullupDisabled {}
