#[doc = "Register `SA1B` reader"]
pub type R = crate::R<Sa1bSpec>;
#[doc = "Register `SA1B` writer"]
pub type W = crate::W<Sa1bSpec>;
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
    pub fn addr(&mut self) -> AddrW<Sa1bSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 1 Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa1b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa1b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa1bSpec;
impl crate::RegisterSpec for Sa1bSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa1b::R`](R) reader structure"]
impl crate::Readable for Sa1bSpec {}
#[doc = "`write(|w| ..)` method takes [`sa1b::W`](W) writer structure"]
impl crate::Writable for Sa1bSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SA1B to value 0"]
impl crate::Resettable for Sa1bSpec {
    const RESET_VALUE: u32 = 0;
}
