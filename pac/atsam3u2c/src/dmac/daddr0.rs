#[doc = "Register `DADDR0` reader"]
pub type R = crate::R<Daddr0Spec>;
#[doc = "Register `DADDR0` writer"]
pub type W = crate::W<Daddr0Spec>;
#[doc = "Field `DADDR` reader - Channel x Destination Address"]
pub type DaddrR = crate::FieldReader<u32>;
#[doc = "Field `DADDR` writer - Channel x Destination Address"]
pub type DaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn daddr(&self) -> DaddrR {
        DaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    #[must_use]
    pub fn daddr(&mut self) -> DaddrW<Daddr0Spec> {
        DaddrW::new(self, 0)
    }
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Daddr0Spec;
impl crate::RegisterSpec for Daddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daddr0::R`](R) reader structure"]
impl crate::Readable for Daddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`daddr0::W`](W) writer structure"]
impl crate::Writable for Daddr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DADDR0 to value 0"]
impl crate::Resettable for Daddr0Spec {
    const RESET_VALUE: u32 = 0;
}
