#[doc = "Register `SA3B` reader"]
pub type R = crate::R<Sa3bSpec>;
#[doc = "Register `SA3B` writer"]
pub type W = crate::W<Sa3bSpec>;
#[doc = "Field `ADDR` reader - "]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - "]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Sa3bSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 3 Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa3b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa3b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa3bSpec;
impl crate::RegisterSpec for Sa3bSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa3b::R`](R) reader structure"]
impl crate::Readable for Sa3bSpec {}
#[doc = "`write(|w| ..)` method takes [`sa3b::W`](W) writer structure"]
impl crate::Writable for Sa3bSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SA3B to value 0"]
impl crate::Resettable for Sa3bSpec {
    const RESET_VALUE: u32 = 0;
}
