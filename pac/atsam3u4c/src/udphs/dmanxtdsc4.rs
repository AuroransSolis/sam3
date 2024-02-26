#[doc = "Register `DMANXTDSC4` reader"]
pub type R = crate::R<Dmanxtdsc4Spec>;
#[doc = "Register `DMANXTDSC4` writer"]
pub type W = crate::W<Dmanxtdsc4Spec>;
#[doc = "Field `NXT_DSC_ADD` reader - Next Descriptor Address"]
pub type NxtDscAddR = crate::FieldReader<u32>;
#[doc = "Field `NXT_DSC_ADD` writer - Next Descriptor Address"]
pub type NxtDscAddW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&self) -> NxtDscAddR {
        NxtDscAddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    #[must_use]
    pub fn nxt_dsc_add(&mut self) -> NxtDscAddW<Dmanxtdsc4Spec> {
        NxtDscAddW::new(self, 0)
    }
}
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmanxtdsc4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmanxtdsc4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmanxtdsc4Spec;
impl crate::RegisterSpec for Dmanxtdsc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmanxtdsc4::R`](R) reader structure"]
impl crate::Readable for Dmanxtdsc4Spec {}
#[doc = "`write(|w| ..)` method takes [`dmanxtdsc4::W`](W) writer structure"]
impl crate::Writable for Dmanxtdsc4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMANXTDSC4 to value 0"]
impl crate::Resettable for Dmanxtdsc4Spec {
    const RESET_VALUE: u32 = 0;
}
