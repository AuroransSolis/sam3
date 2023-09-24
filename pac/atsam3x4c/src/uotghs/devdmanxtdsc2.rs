#[doc = "Register `DEVDMANXTDSC2` reader"]
pub type R = crate::R<DEVDMANXTDSC2_SPEC>;
#[doc = "Register `DEVDMANXTDSC2` writer"]
pub type W = crate::W<DEVDMANXTDSC2_SPEC>;
#[doc = "Field `NXT_DSC_ADD` reader - Next Descriptor Address"]
pub type NXT_DSC_ADD_R = crate::FieldReader<u32>;
#[doc = "Field `NXT_DSC_ADD` writer - Next Descriptor Address"]
pub type NXT_DSC_ADD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&self) -> NXT_DSC_ADD_R {
        NXT_DSC_ADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    #[must_use]
    pub fn nxt_dsc_add(&mut self) -> NXT_DSC_ADD_W<DEVDMANXTDSC2_SPEC, 0> {
        NXT_DSC_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVDMANXTDSC2_SPEC;
impl crate::RegisterSpec for DEVDMANXTDSC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdmanxtdsc2::R`](R) reader structure"]
impl crate::Readable for DEVDMANXTDSC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devdmanxtdsc2::W`](W) writer structure"]
impl crate::Writable for DEVDMANXTDSC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVDMANXTDSC2 to value 0"]
impl crate::Resettable for DEVDMANXTDSC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
