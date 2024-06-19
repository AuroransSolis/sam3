#[doc = "Register `SA4B` reader"]
pub type R = crate::R<Sa4bSpec>;
#[doc = "Register `SA4B` writer"]
pub type W = crate::W<Sa4bSpec>;
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
    pub fn addr(&mut self) -> AddrW<Sa4bSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 4 Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa4b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa4b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa4bSpec;
impl crate::RegisterSpec for Sa4bSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa4b::R`](R) reader structure"]
impl crate::Readable for Sa4bSpec {}
#[doc = "`write(|w| ..)` method takes [`sa4b::W`](W) writer structure"]
impl crate::Writable for Sa4bSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SA4B to value 0"]
impl crate::Resettable for Sa4bSpec {
    const RESET_VALUE: u32 = 0;
}
