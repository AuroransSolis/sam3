#[doc = "Register `DEVDMANXTDSC1` reader"]
pub type R = crate::R<Devdmanxtdsc1Spec>;
#[doc = "Register `DEVDMANXTDSC1` writer"]
pub type W = crate::W<Devdmanxtdsc1Spec>;
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
    pub fn nxt_dsc_add(&mut self) -> NxtDscAddW<Devdmanxtdsc1Spec> {
        NxtDscAddW::new(self, 0)
    }
}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Devdmanxtdsc1Spec;
impl crate::RegisterSpec for Devdmanxtdsc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdmanxtdsc1::R`](R) reader structure"]
impl crate::Readable for Devdmanxtdsc1Spec {}
#[doc = "`write(|w| ..)` method takes [`devdmanxtdsc1::W`](W) writer structure"]
impl crate::Writable for Devdmanxtdsc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVDMANXTDSC1 to value 0"]
impl crate::Resettable for Devdmanxtdsc1Spec {
    const RESET_VALUE: u32 = 0;
}
