#[doc = "Register `USF` reader"]
pub type R = crate::R<USF_SPEC>;
#[doc = "Register `USF` writer"]
pub type W = crate::W<USF_SPEC>;
#[doc = "Field `USF` reader - Undersize frames"]
pub type USF_R = crate::FieldReader;
#[doc = "Field `USF` writer - Undersize frames"]
pub type USF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Undersize frames"]
    #[inline(always)]
    pub fn usf(&self) -> USF_R {
        USF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Undersize frames"]
    #[inline(always)]
    #[must_use]
    pub fn usf(&mut self) -> USF_W<USF_SPEC> {
        USF_W::new(self, 0)
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
#[doc = "Undersize Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USF_SPEC;
impl crate::RegisterSpec for USF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usf::R`](R) reader structure"]
impl crate::Readable for USF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usf::W`](W) writer structure"]
impl crate::Writable for USF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USF to value 0"]
impl crate::Resettable for USF_SPEC {
    const RESET_VALUE: u32 = 0;
}
