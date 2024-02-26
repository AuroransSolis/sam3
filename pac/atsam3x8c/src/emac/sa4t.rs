#[doc = "Register `SA4T` reader"]
pub type R = crate::R<Sa4tSpec>;
#[doc = "Register `SA4T` writer"]
pub type W = crate::W<Sa4tSpec>;
#[doc = "Field `ADDR` reader - "]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - "]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Sa4tSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 4 Top Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa4t::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa4t::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa4tSpec;
impl crate::RegisterSpec for Sa4tSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa4t::R`](R) reader structure"]
impl crate::Readable for Sa4tSpec {}
#[doc = "`write(|w| ..)` method takes [`sa4t::W`](W) writer structure"]
impl crate::Writable for Sa4tSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SA4T to value 0"]
impl crate::Resettable for Sa4tSpec {
    const RESET_VALUE: u32 = 0;
}
