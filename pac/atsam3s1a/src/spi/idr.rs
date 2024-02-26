#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Disable"]
pub type RdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRE` writer - SPI Transmit Data Register Empty Interrupt Disable"]
pub type TdreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` writer - Mode Fault Error Interrupt Disable"]
pub type ModfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Disable"]
pub type OvresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Disable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Disable"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Disable"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Disable"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSR` writer - NSS Rising Interrupt Disable"]
pub type NssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Disable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDES` writer - Underrun Error Interrupt Disable"]
pub type UndesW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rdrf(&mut self) -> RdrfW<IdrSpec> {
        RdrfW::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TdreW<IdrSpec> {
        TdreW::new(self, 1)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn modf(&mut self) -> ModfW<IdrSpec> {
        ModfW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovres(&mut self) -> OvresW<IdrSpec> {
        OvresW::new(self, 3)
    }
    #[doc = "Bit 4 - End of Receive Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IdrSpec> {
        EndrxW::new(self, 4)
    }
    #[doc = "Bit 5 - End of Transmit Buffer Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IdrSpec> {
        EndtxW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Buffer Full Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RxbuffW<IdrSpec> {
        RxbuffW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TxbufeW<IdrSpec> {
        TxbufeW::new(self, 7)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nssr(&mut self) -> NssrW<IdrSpec> {
        NssrW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TxemptyW<IdrSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn undes(&mut self) -> UndesW<IdrSpec> {
        UndesW::new(self, 10)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
