#[doc = "Register `PFR` reader"]
pub type R = crate::R<PFR_SPEC>;
#[doc = "Register `PFR` writer"]
pub type W = crate::W<PFR_SPEC>;
#[doc = "Field `FROK` reader - Pause Frames received OK"]
pub type FROK_R = crate::FieldReader<u16>;
#[doc = "Field `FROK` writer - Pause Frames received OK"]
pub type FROK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Frames received OK"]
    #[inline(always)]
    pub fn frok(&self) -> FROK_R {
        FROK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pause Frames received OK"]
    #[inline(always)]
    #[must_use]
    pub fn frok(&mut self) -> FROK_W<PFR_SPEC, 0> {
        FROK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pause Frames Received Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR_SPEC;
impl crate::RegisterSpec for PFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr::R`](R) reader structure"]
impl crate::Readable for PFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr::W`](W) writer structure"]
impl crate::Writable for PFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFR to value 0"]
impl crate::Resettable for PFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
