#[doc = "Register `SCDR` reader"]
pub type R = crate::R<ScdrSpec>;
#[doc = "Register `SCDR` writer"]
pub type W = crate::W<ScdrSpec>;
#[doc = "Field `DIV` reader - Slow Clock Divider Selection for Debouncing"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Slow Clock Divider Selection for Debouncing"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Slow Clock Divider Selection for Debouncing"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Slow Clock Divider Selection for Debouncing"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<ScdrSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "Slow Clock Divider Debouncing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScdrSpec;
impl crate::RegisterSpec for ScdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scdr::R`](R) reader structure"]
impl crate::Readable for ScdrSpec {}
#[doc = "`write(|w| ..)` method takes [`scdr::W`](W) writer structure"]
impl crate::Writable for ScdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCDR to value 0"]
impl crate::Resettable for ScdrSpec {
    const RESET_VALUE: u32 = 0;
}
