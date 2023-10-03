#[doc = "Register `CMPVUPD0` writer"]
pub type W = crate::W<CMPVUPD0_SPEC>;
#[doc = "Field `CVUPD` writer - Comparison x Value Update"]
pub type CVUPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `CVMUPD` writer - Comparison x Value Mode Update"]
pub type CVMUPD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:23 - Comparison x Value Update"]
    #[inline(always)]
    #[must_use]
    pub fn cvupd(&mut self) -> CVUPD_W<CMPVUPD0_SPEC, 0> {
        CVUPD_W::new(self)
    }
    #[doc = "Bit 24 - Comparison x Value Mode Update"]
    #[inline(always)]
    #[must_use]
    pub fn cvmupd(&mut self) -> CVMUPD_W<CMPVUPD0_SPEC, 24> {
        CVMUPD_W::new(self)
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
#[doc = "PWM Comparison 0 Value Update Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpvupd0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPVUPD0_SPEC;
impl crate::RegisterSpec for CMPVUPD0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmpvupd0::W`](W) writer structure"]
impl crate::Writable for CMPVUPD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
