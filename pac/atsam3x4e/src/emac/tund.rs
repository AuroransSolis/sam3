#[doc = "Register `TUND` reader"]
pub type R = crate::R<TUND_SPEC>;
#[doc = "Register `TUND` writer"]
pub type W = crate::W<TUND_SPEC>;
#[doc = "Field `TUND` reader - Transmit Underruns"]
pub type TUND_R = crate::FieldReader;
#[doc = "Field `TUND` writer - Transmit Underruns"]
pub type TUND_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Underruns"]
    #[inline(always)]
    pub fn tund(&self) -> TUND_R {
        TUND_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Underruns"]
    #[inline(always)]
    #[must_use]
    pub fn tund(&mut self) -> TUND_W<TUND_SPEC> {
        TUND_W::new(self, 0)
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
#[doc = "Transmit Underrun Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tund::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tund::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TUND_SPEC;
impl crate::RegisterSpec for TUND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tund::R`](R) reader structure"]
impl crate::Readable for TUND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tund::W`](W) writer structure"]
impl crate::Writable for TUND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TUND to value 0"]
impl crate::Resettable for TUND_SPEC {
    const RESET_VALUE: u32 = 0;
}
