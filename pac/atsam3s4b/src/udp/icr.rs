#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `RXSUSP` writer - Clear UDP Suspend Interrupt"]
pub type RxsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRSM` writer - Clear UDP Resume Interrupt"]
pub type RxrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRSM` writer - "]
pub type ExtrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFINT` writer - Clear Start Of Frame Interrupt"]
pub type SofintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDBUSRES` writer - Clear End of Bus Reset Interrupt"]
pub type EndbusresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` writer - Clear Wakeup Interrupt"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - Clear UDP Suspend Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxsusp(&mut self) -> RxsuspW<IcrSpec> {
        RxsuspW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear UDP Resume Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsm(&mut self) -> RxrsmW<IcrSpec> {
        RxrsmW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn extrsm(&mut self) -> ExtrsmW<IcrSpec> {
        ExtrsmW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear Start Of Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sofint(&mut self) -> SofintW<IcrSpec> {
        SofintW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear End of Bus Reset Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn endbusres(&mut self) -> EndbusresW<IcrSpec> {
        EndbusresW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Wakeup Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<IcrSpec> {
        WakeupW::new(self, 13)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
