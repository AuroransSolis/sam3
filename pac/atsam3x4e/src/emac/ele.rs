#[doc = "Register `ELE` reader"]
pub type R = crate::R<EleSpec>;
#[doc = "Register `ELE` writer"]
pub type W = crate::W<EleSpec>;
#[doc = "Field `EXL` reader - Excessive Length Errors"]
pub type ExlR = crate::FieldReader;
#[doc = "Field `EXL` writer - Excessive Length Errors"]
pub type ExlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Excessive Length Errors"]
    #[inline(always)]
    pub fn exl(&self) -> ExlR {
        ExlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Excessive Length Errors"]
    #[inline(always)]
    #[must_use]
    pub fn exl(&mut self) -> ExlW<EleSpec> {
        ExlW::new(self, 0)
    }
}
#[doc = "Excessive Length Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ele::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ele::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EleSpec;
impl crate::RegisterSpec for EleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ele::R`](R) reader structure"]
impl crate::Readable for EleSpec {}
#[doc = "`write(|w| ..)` method takes [`ele::W`](W) writer structure"]
impl crate::Writable for EleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ELE to value 0"]
impl crate::Resettable for EleSpec {
    const RESET_VALUE: u32 = 0;
}
