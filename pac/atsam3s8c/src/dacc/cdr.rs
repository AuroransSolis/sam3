#[doc = "Register `CDR` writer"]
pub type W = crate::W<CDR_SPEC>;
#[doc = "Field `DATA` writer - Data to Convert"]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Data to Convert"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<CDR_SPEC, 0> {
        DATA_W::new(self)
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
#[doc = "Conversion Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cdr::W`](W) writer structure"]
impl crate::Writable for CDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
