#[doc = "Register `SA1T` reader"]
pub type R = crate::R<Sa1tSpec>;
#[doc = "Register `SA1T` writer"]
pub type W = crate::W<Sa1tSpec>;
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
    pub fn addr(&mut self) -> AddrW<Sa1tSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 1 Top Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sa1t::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa1t::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa1tSpec;
impl crate::RegisterSpec for Sa1tSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa1t::R`](R) reader structure"]
impl crate::Readable for Sa1tSpec {}
#[doc = "`write(|w| ..)` method takes [`sa1t::W`](W) writer structure"]
impl crate::Writable for Sa1tSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SA1T to value 0"]
impl crate::Resettable for Sa1tSpec {
    const RESET_VALUE: u32 = 0;
}
