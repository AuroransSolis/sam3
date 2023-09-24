#[doc = "Register `HSTDMANXTDSC3` reader"]
pub type R = crate::R<HSTDMANXTDSC3_SPEC>;
#[doc = "Register `HSTDMANXTDSC3` writer"]
pub type W = crate::W<HSTDMANXTDSC3_SPEC>;
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
    pub fn nxt_dsc_add(&mut self) -> NXT_DSC_ADD_W<HSTDMANXTDSC3_SPEC, 0> {
        NXT_DSC_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstdmanxtdsc3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstdmanxtdsc3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTDMANXTDSC3_SPEC;
impl crate::RegisterSpec for HSTDMANXTDSC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstdmanxtdsc3::R`](R) reader structure"]
impl crate::Readable for HSTDMANXTDSC3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstdmanxtdsc3::W`](W) writer structure"]
impl crate::Writable for HSTDMANXTDSC3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTDMANXTDSC3 to value 0"]
impl crate::Resettable for HSTDMANXTDSC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
