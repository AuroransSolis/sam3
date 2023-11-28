#[doc = "Register `DEVICR` writer"]
pub type W = crate::W<DEVICR_SPEC>;
#[doc = "Field `SUSPC` writer - Suspend Interrupt Clear"]
pub type SUSPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSOFC` writer - Micro Start of Frame Interrupt Clear"]
pub type MSOFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFC` writer - Start of Frame Interrupt Clear"]
pub type SOFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSTC` writer - End of Reset Interrupt Clear"]
pub type EORSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPC` writer - Wake-Up Interrupt Clear"]
pub type WAKEUPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSMC` writer - End of Resume Interrupt Clear"]
pub type EORSMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSMC` writer - Upstream Resume Interrupt Clear"]
pub type UPRSMC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn suspc(&mut self) -> SUSPC_W<DEVICR_SPEC> {
        SUSPC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn msofc(&mut self) -> MSOFC_W<DEVICR_SPEC> {
        MSOFC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sofc(&mut self) -> SOFC_W<DEVICR_SPEC> {
        SOFC_W::new(self, 2)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eorstc(&mut self) -> EORSTC_W<DEVICR_SPEC> {
        EORSTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupc(&mut self) -> WAKEUPC_W<DEVICR_SPEC> {
        WAKEUPC_W::new(self, 4)
    }
    #[doc = "Bit 5 - End of Resume Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eorsmc(&mut self) -> EORSMC_W<DEVICR_SPEC> {
        EORSMC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uprsmc(&mut self) -> UPRSMC_W<DEVICR_SPEC> {
        UPRSMC_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Global Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICR_SPEC;
impl crate::RegisterSpec for DEVICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devicr::W`](W) writer structure"]
impl crate::Writable for DEVICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
