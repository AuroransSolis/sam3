#[doc = "Register `SA3T` reader"]
pub type R = crate::R<Sa3tSpec>;
#[doc = "Register `SA3T` writer"]
pub type W = crate::W<Sa3tSpec>;
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
    pub fn addr(&mut self) -> AddrW<Sa3tSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 3 Top Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa3t::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa3t::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa3tSpec;
impl crate::RegisterSpec for Sa3tSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa3t::R`](R) reader structure"]
impl crate::Readable for Sa3tSpec {}
#[doc = "`write(|w| ..)` method takes [`sa3t::W`](W) writer structure"]
impl crate::Writable for Sa3tSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SA3T to value 0"]
impl crate::Resettable for Sa3tSpec {
    const RESET_VALUE: u32 = 0;
}
