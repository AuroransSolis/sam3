#[doc = "Register `SA2B` reader"]
pub type R = crate::R<Sa2bSpec>;
#[doc = "Register `SA2B` writer"]
pub type W = crate::W<Sa2bSpec>;
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
    pub fn addr(&mut self) -> AddrW<Sa2bSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 2 Bottom Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa2b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa2b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa2bSpec;
impl crate::RegisterSpec for Sa2bSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa2b::R`](R) reader structure"]
impl crate::Readable for Sa2bSpec {}
#[doc = "`write(|w| ..)` method takes [`sa2b::W`](W) writer structure"]
impl crate::Writable for Sa2bSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SA2B to value 0"]
impl crate::Resettable for Sa2bSpec {
    const RESET_VALUE: u32 = 0;
}
