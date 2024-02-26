#[doc = "Register `DMANXTDSC2` reader"]
pub type R = crate::R<Dmanxtdsc2Spec>;
#[doc = "Register `DMANXTDSC2` writer"]
pub type W = crate::W<Dmanxtdsc2Spec>;
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
    pub fn nxt_dsc_add(&mut self) -> NxtDscAddW<Dmanxtdsc2Spec> {
        NxtDscAddW::new(self, 0)
    }
}
#[doc = "UDPHS DMA Next Descriptor Address Register (channel = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmanxtdsc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmanxtdsc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmanxtdsc2Spec;
impl crate::RegisterSpec for Dmanxtdsc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmanxtdsc2::R`](R) reader structure"]
impl crate::Readable for Dmanxtdsc2Spec {}
#[doc = "`write(|w| ..)` method takes [`dmanxtdsc2::W`](W) writer structure"]
impl crate::Writable for Dmanxtdsc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMANXTDSC2 to value 0"]
impl crate::Resettable for Dmanxtdsc2Spec {
    const RESET_VALUE: u32 = 0;
}
