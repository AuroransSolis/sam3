#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Enable"]
pub type RdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRE` writer - SPI Transmit Data Register Empty Interrupt Enable"]
pub type TdreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` writer - Mode Fault Error Interrupt Enable"]
pub type ModfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Enable"]
pub type OvresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Enable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Enable"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Enable"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSR` writer - NSS Rising Interrupt Enable"]
pub type NssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDES` writer - Underrun Error Interrupt Enable"]
pub type UndesW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdrf(&mut self) -> RdrfW<IerSpec> {
        RdrfW::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TdreW<IerSpec> {
        TdreW::new(self, 1)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn modf(&mut self) -> ModfW<IerSpec> {
        ModfW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovres(&mut self) -> OvresW<IerSpec> {
        OvresW::new(self, 3)
    }
    #[doc = "Bit 4 - End of Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IerSpec> {
        EndrxW::new(self, 4)
    }
    #[doc = "Bit 5 - End of Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IerSpec> {
        EndtxW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<IerSpec> {
        RxbuffW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TxbufeW<IerSpec> {
        TxbufeW::new(self, 7)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nssr(&mut self) -> NssrW<IerSpec> {
        NssrW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<IerSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn undes(&mut self) -> UndesW<IerSpec> {
        UndesW::new(self, 10)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
