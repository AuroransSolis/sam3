#[doc = "Register `CMPVUPD5` writer"]
pub type W = crate::W<CMPVUPD5_SPEC>;
#[doc = "Field `CVUPD` writer - Comparison x Value Update"]
pub type CVUPD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `CVMUPD` writer - Comparison x Value Mode Update"]
pub type CVMUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:23 - Comparison x Value Update"]
    #[inline(always)]
    #[must_use]
    pub fn cvupd(&mut self) -> CVUPD_W<CMPVUPD5_SPEC> {
        CVUPD_W::new(self, 0)
    }
    #[doc = "Bit 24 - Comparison x Value Mode Update"]
    #[inline(always)]
    #[must_use]
    pub fn cvmupd(&mut self) -> CVMUPD_W<CMPVUPD5_SPEC> {
        CVMUPD_W::new(self, 24)
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
#[doc = "PWM Comparison 5 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPVUPD5_SPEC;
impl crate::RegisterSpec for CMPVUPD5_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmpvupd5::W`](W) writer structure"]
impl crate::Writable for CMPVUPD5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
