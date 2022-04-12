use super::Pioc;
use core::marker::PhantomData;

pub trait PinId {}
pub trait LineCfg {}
pub trait OutputWriteCfg {}
pub trait OutputCfg {}
pub trait PullupResistorCfg {}
pub trait InterruptCfg {}
pub trait MultiDriverCfg {}
pub trait ABSelectCfg {}
pub trait OutputDataCfg {}
pub trait InputFilterCfg {}
pub trait InputFilterClockCfg {}
pub trait AdditionalInterruptModesCfg {}
pub trait EdgeLevelCfg {}
pub trait FallLowRiseHighCfg {}

pub struct Pin<
    Pio,
    Pid,
    Line = PeripheralControlled,
    Outw = OutputWriteEnabled,
    Otpt = OutputEnabled,
    Pupr = PullUpDisabled,
    Irpt = InterruptDisabled,
    Mdvr = MultiDriverDisabled,
    Absl = PeripheralA,
    Odta = ClearOutput,
    Filt = InputFilterDisabled,
    Flck = SystemClockGlitchFilter,
    Aint = AdditionalInterruptModesDisabled,
    Edlv = DetectEdges,
    Frlh = DetectRisingEdgeHighLevel,
> where
    Pio: Pioc,
    Pid: PinId,
    Line: LineCfg,
    Outw: OutputWriteCfg,
    Otpt: OutputCfg,
    Pupr: PullupResistorCfg,
    Irpt: InterruptCfg,
    Mdvr: MultiDriverCfg,
    Absl: ABSelectCfg,
    Odta: OutputDataCfg,
    Filt: InputFilterCfg,
    Flck: InputFilterClockCfg,
    Aint: AdditionalInterruptModesCfg,
    Edlv: EdgeLevelCfg,
    Frlh: FallLowRiseHighCfg,
{
    _pio: PhantomData<Pio>,
    _pid: PhantomData<Pid>,
    _line: PhantomData<Line>,
    _outw: PhantomData<Outw>,
    _otpt: PhantomData<Otpt>,
    _pupr: PhantomData<Pupr>,
    _irpt: PhantomData<Irpt>,
    _mdvr: PhantomData<Mdvr>,
    _absl: PhantomData<Absl>,
    _odta: PhantomData<Odta>,
    _filt: PhantomData<Filt>,
    _flck: PhantomData<Flck>,
    _aint: PhantomData<Aint>,
    _edlv: PhantomData<Edlv>,
    _frlh: PhantomData<Frlh>,
}

// Yeah, I could get away without using this one, but for me it really helps visually organise the
// types I'm using for my type-level enums better. What so I can keep track of what's where and all.
macro_rules! make_cfg_types {
    ($($cfg_trait:ident {
        $($(#[$meta:meta])*$cfg_type:ident),+$(,)?
    }),+$(,)?) => {
        $(
            $(
                $(#[$meta])*
                pub struct $cfg_type;
                impl $cfg_trait for $cfg_type {}
            )+
        )+
    };
}

// See?
make_cfg_types! {
    LineCfg {
        /// Allow the peripheral to control this I/O line.
        PeripheralControlled,
        /// Allow the PIO controller to control this I/O.
        PioControlled,
    },
    OutputWriteCfg {
        /// Enable setting I/O line data through the `PIO_ODSR` register.
        OutputWriteEnabled,
        /// Disable setting I/O line data through the `PIO_ODSR` register.
        OutputWriteDisabled,
    },
    OutputCfg {
        /// Enable output from this I/O line.
        OutputEnabled,
        /// Disable output from this I/O line.
        OutputDisabled,
    },
    PullupResistorCfg {
        /// Enable the pull-up resistor on an I/O line.
        PullUpEnabled,
        /// Disable the pull-up resistor on an I/O line.
        PullUpDisabled,
    },
    InterruptCfg {
        /// Enable the Input Change Interrupt on an I/O line.
        InterruptEnabled,
        /// Disable the Input Change Interrupt on an I/O line.
        InterruptDisabled,
    },
    MultiDriverCfg {
        /// Set an I/O line to open drain, permitting several drivers to be connected. Connected
        /// drivers can only drive the line low, and an external pull-up resistor (or enabling the
        /// internal one) is generally required to guarantee a high level on the line.
        MultiDriverEnabled,
        /// Disable Multi Drive control on an I/O line.
        MultiDriverDisabled,
    },
    ABSelectCfg {
        /// Allow output from peripheral A on an I/O line.
        PeripheralA,
        /// Allow output from peripheral B on an I/O line.
        PeripheralB,
    },
    OutputDataCfg {
        /// Drive the output of an I/O line high.
        SetOutput,
        /// Pull the output of an I/O line low.
        ClearOutput,
    },
    InputFilterCfg {
        /// Enable the glitch input filter on an I/O line.
        InputFilterEnabled,
        /// Disable the glitch input filter on an I/O line.
        InputFilterDisabled,
    },
    InputFilterClockCfg {
        /// Set an I/O line's input filter to use the system clock glitch filter.
        SystemClockGlitchFilter,
        /// Set an I/O line's input filter to use the debouncing filter.
        DebouncingFilter,
    },
    AdditionalInterruptModesCfg {
        /// Enable interrupts from the event detector on an I/O line.
        AdditionalInterruptModesEnabled,
        /// Disable interrupts from the event detector on an I/O line.
        AdditionalInterruptModesDisabled,
    },
    EdgeLevelCfg {
        /// Configure an I/O line's event detector to detect edges.
        DetectEdges,
        /// Configure an I/O line's event detector to detect levels.
        DetectLevels,
    },
    FallLowRiseHighCfg {
        /// Configure an I/O line's event detector to detect falling edges or low levels depending
        /// on whether it is configured to detect edges or levels.
        DetectFallingEdgeLowLevel,
        /// Configure an I/O line's event detector to detect rising edges or high levels depending
        /// on whether it is configured to detect edges or levels.
        DetectRisingEdgeHighLevel,
    },
}

impl<Pio, Pid, Line, Outw, Otpt, Pupr, Irpt, Mdvr, Absl, Odta, Filt, Flck, Aint, Edlv, Frlh>
    Pin<Pio, Pid, Line, Outw, Otpt, Pupr, Irpt, Mdvr, Absl, Odta, Filt, Flck, Aint, Edlv, Frlh>
where
    Pio: Pioc,
    Pid: PinId,
    Line: LineCfg,
    Outw: OutputWriteCfg,
    Otpt: OutputCfg,
    Pupr: PullupResistorCfg,
    Irpt: InterruptCfg,
    Mdvr: MultiDriverCfg,
    Absl: ABSelectCfg,
    Odta: OutputDataCfg,
    Filt: InputFilterCfg,
    Flck: InputFilterClockCfg,
    Aint: AdditionalInterruptModesCfg,
    Edlv: EdgeLevelCfg,
    Frlh: FallLowRiseHighCfg,
{
    // Define methods to change pin configurations here.
}
