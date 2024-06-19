#[doc = "Register `DEVICR` writer"]
pub type W = crate::W<DevicrSpec>;
#[doc = "Field `SUSPC` writer - Suspend Interrupt Clear"]
pub type SuspcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSOFC` writer - Micro Start of Frame Interrupt Clear"]
pub type MsofcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFC` writer - Start of Frame Interrupt Clear"]
pub type SofcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSTC` writer - End of Reset Interrupt Clear"]
pub type EorstcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPC` writer - Wake-Up Interrupt Clear"]
pub type WakeupcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSMC` writer - End of Resume Interrupt Clear"]
pub type EorsmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSMC` writer - Upstream Resume Interrupt Clear"]
pub type UprsmcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn suspc(&mut self) -> SuspcW<DevicrSpec> {
        SuspcW::new(self, 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn msofc(&mut self) -> MsofcW<DevicrSpec> {
        MsofcW::new(self, 1)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sofc(&mut self) -> SofcW<DevicrSpec> {
        SofcW::new(self, 2)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eorstc(&mut self) -> EorstcW<DevicrSpec> {
        EorstcW::new(self, 3)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupc(&mut self) -> WakeupcW<DevicrSpec> {
        WakeupcW::new(self, 4)
    }
    #[doc = "Bit 5 - End of Resume Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eorsmc(&mut self) -> EorsmcW<DevicrSpec> {
        EorsmcW::new(self, 5)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uprsmc(&mut self) -> UprsmcW<DevicrSpec> {
        UprsmcW::new(self, 6)
    }
}
#[doc = "Device Global Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevicrSpec;
impl crate::RegisterSpec for DevicrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devicr::W`](W) writer structure"]
impl crate::Writable for DevicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
