use crate::pio::IsPioc;
use core::marker::PhantomData;

pub struct DynPin<Pioc, Pid>
where
    Pioc: IsPioc,
    Pid: PinId,
{
    pioc: PhantomData<Pioc>,
    id: PhantomData<Pid>,
}

impl<Pioc, Pid> DynPin<Pioc, Pid>
where
    Pioc: IsPioc,
    Pid: PinId,
{
    pub(crate) unsafe fn new() -> Self {
        DynPin {
            pioc: PhantomData,
            id: PhantomData,
        }
    }
}
