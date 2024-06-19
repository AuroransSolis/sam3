#[doc = "Register `ALE` reader"]
pub type R = crate::R<AleSpec>;
#[doc = "Register `ALE` writer"]
pub type W = crate::W<AleSpec>;
#[doc = "Field `ALE` reader - Alignment Errors"]
pub type AleR = crate::FieldReader;
#[doc = "Field `ALE` writer - Alignment Errors"]
pub type AleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Alignment Errors"]
    #[inline(always)]
    pub fn ale(&self) -> AleR {
        AleR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alignment Errors"]
    #[inline(always)]
    #[must_use]
    pub fn ale(&mut self) -> AleW<AleSpec> {
        AleW::new(self, 0)
    }
}
#[doc = "Alignment Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleSpec;
impl crate::RegisterSpec for AleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale::R`](R) reader structure"]
impl crate::Readable for AleSpec {}
#[doc = "`write(|w| ..)` method takes [`ale::W`](W) writer structure"]
impl crate::Writable for AleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE to value 0"]
impl crate::Resettable for AleSpec {
    const RESET_VALUE: u32 = 0;
}
