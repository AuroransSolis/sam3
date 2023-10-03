#[doc = "Register `OCMS` reader"]
pub type R = crate::R<OCMS_SPEC>;
#[doc = "Register `OCMS` writer"]
pub type W = crate::W<OCMS_SPEC>;
#[doc = "Field `SDR_SE` reader - SDRAM Memory Controller Scrambling Enable"]
pub type SDR_SE_R = crate::BitReader;
#[doc = "Field `SDR_SE` writer - SDRAM Memory Controller Scrambling Enable"]
pub type SDR_SE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SDRAM Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn sdr_se(&self) -> SDR_SE_R {
        SDR_SE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDRAM Memory Controller Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdr_se(&mut self) -> SDR_SE_W<OCMS_SPEC, 0> {
        SDR_SE_W::new(self)
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
#[doc = "SDRAMC OCMS Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocms::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocms::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCMS_SPEC;
impl crate::RegisterSpec for OCMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocms::R`](R) reader structure"]
impl crate::Readable for OCMS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocms::W`](W) writer structure"]
impl crate::Writable for OCMS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCMS to value 0"]
impl crate::Resettable for OCMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
