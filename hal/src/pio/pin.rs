use crate::pio::{
    filter::InputFilterCfg, interrupt::InterruptCfg, peripheral::MultiDriverCfg, IsPio,
};
use core::marker::PhantomData;

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

/*
unsure if this is the right way to do it - maybe reverse?
what i mean by this is instead of

type Pwm0<Pupr, Irpt, Filt> = Pin<PioA, Pa0, MultiDriverDisabled<...>, Pupr, Irpt, Filt>;

instead have

pub trait Pwm0: ConfigurePioControl + ConfigureFunctionSelect {
    type Pwm0;

    fn to_pwm0(self) -> Result<Self::Pwm0, (Self, PioError)>;
    unsafe fn to_pwm0_unchecked(self) -> Self::Pwm0;
}

impl<Line, Pupr, Irpt, Filt> Pwm0 for Pin<PioA, Pa0, MultiDriverDisabled<Line>, Pupr, Irpt, Filt>
where
    Pin<PioA, Pa0, MultiDriverDisabled<Line>, Pupr, Irpt, Filt>: ConfigurePioControl
        + ConfigureFunctionSelect,
{
    type Pwm0 =
        Pin<PioA, Pa0, MultiDriverDisabled<PeripheralControlled<PeripheralA>>, Pupr, Irpt, Filt>;

    fn to_pwm0(self) -> Result<Self::Pwm0, (Self, PioError)> {
        self.peripheral_controlled()?.peripheral_a()
    }

    unsafe fn to_pwm0_unchecked(self) -> Self::Pwm0 {
        self.peripheral_controlled_unchecked().peripheral_a_unchecked()
    }
}

and so on

macro_rules! def_peripheral_multiplex {
    (
        $pio:ty {
            $($pid:ty: $opts:tt),+$(,)?
        }
    ) => {
        $(
            def_peripheral_multiplex! {
                @expand $pio, $pid, $opts
            }
        )+
    };
    (@expand $pio:ty, $pid:ty, [$($sel:tt $name:ident),+$(,)?]) => {
        $(
            def_peripheral_multiplex! {
                @def $pio, $pid, $sel, $name
            }
        )+
    };
    (@def $pio:ty, $pid:ty, $suffix:tt, $name:ident) => {
        paste::paste! {
            pub type $name<Pupr, Irpt, Filt> = crate::pio::pin::Pin<
                $pio,
                $pid,
                crate::pio::peripheral::MultiDriverDisabled<
                    crate::pio::peripheral::PeripheralControlled<
                        crate::pio::peripheral::[<Peripheral $suffix:upper>]
                    >,
                >,
                Pupr,
                Irpt,
                Filt,
            >;
            pub type [<$name MD>]<Otpt, Pupr, Irpt, Filt> = crate::pio::pin::Pin<
                $pio,
                $pid,
                crate::pio::peripheral::MultiDriverEnabled<
                    crate::pio::peripheral::PeripheralControlled<
                        crate::pio::peripheral::[<Peripheral $suffix:upper>]
                    >,
                    Otpt,
                >,
                Pupr,
                Irpt,
                Filt,
            >;
        }
    };
}

pub(crate) use def_peripheral_multiplex;
*/
