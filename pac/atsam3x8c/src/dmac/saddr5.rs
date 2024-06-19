#[doc = "Register `SADDR5` reader"]
pub type R = crate::R<Saddr5Spec>;
#[doc = "Register `SADDR5` writer"]
pub type W = crate::W<Saddr5Spec>;
#[doc = "Field `SADDR` reader - Channel x Source Address"]
pub type SaddrR = crate::FieldReader<u32>;
#[doc = "Field `SADDR` writer - Channel x Source Address"]
pub type SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn saddr(&self) -> SaddrR {
        SaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    #[must_use]
    pub fn saddr(&mut self) -> SaddrW<Saddr5Spec> {
        SaddrW::new(self, 0)
    }
}
#[doc = "DMAC Channel Source Address Register (ch_num = 5)\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saddr5Spec;
impl crate::RegisterSpec for Saddr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr5::R`](R) reader structure"]
impl crate::Readable for Saddr5Spec {}
#[doc = "`write(|w| ..)` method takes [`saddr5::W`](W) writer structure"]
impl crate::Writable for Saddr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SADDR5 to value 0"]
impl crate::Resettable for Saddr5Spec {
    const RESET_VALUE: u32 = 0;
}
