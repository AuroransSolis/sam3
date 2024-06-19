#[doc = "Register `SADDR2` reader"]
pub type R = crate::R<Saddr2Spec>;
#[doc = "Register `SADDR2` writer"]
pub type W = crate::W<Saddr2Spec>;
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
    pub fn saddr(&mut self) -> SaddrW<Saddr2Spec> {
        SaddrW::new(self, 0)
    }
}
#[doc = "DMAC Channel Source Address Register (ch_num = 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Saddr2Spec;
impl crate::RegisterSpec for Saddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr2::R`](R) reader structure"]
impl crate::Readable for Saddr2Spec {}
#[doc = "`write(|w| ..)` method takes [`saddr2::W`](W) writer structure"]
impl crate::Writable for Saddr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SADDR2 to value 0"]
impl crate::Resettable for Saddr2Spec {
    const RESET_VALUE: u32 = 0;
}
