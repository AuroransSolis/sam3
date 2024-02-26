#[doc = "Register `DADDR2` reader"]
pub type R = crate::R<Daddr2Spec>;
#[doc = "Register `DADDR2` writer"]
pub type W = crate::W<Daddr2Spec>;
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
    pub fn daddr(&mut self) -> DaddrW<Daddr2Spec> {
        DaddrW::new(self, 0)
    }
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Daddr2Spec;
impl crate::RegisterSpec for Daddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daddr2::R`](R) reader structure"]
impl crate::Readable for Daddr2Spec {}
#[doc = "`write(|w| ..)` method takes [`daddr2::W`](W) writer structure"]
impl crate::Writable for Daddr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DADDR2 to value 0"]
impl crate::Resettable for Daddr2Spec {
    const RESET_VALUE: u32 = 0;
}
