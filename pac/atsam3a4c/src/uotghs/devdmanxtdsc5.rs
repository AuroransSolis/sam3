#[doc = "Register `DEVDMANXTDSC5` reader"]
pub type R = crate::R<DEVDMANXTDSC5_SPEC>;
#[doc = "Register `DEVDMANXTDSC5` writer"]
pub type W = crate::W<DEVDMANXTDSC5_SPEC>;
#[doc = "Field `NXT_DSC_ADD` reader - Next Descriptor Address"]
pub type NXT_DSC_ADD_R = crate::FieldReader<u32>;
#[doc = "Field `NXT_DSC_ADD` writer - Next Descriptor Address"]
pub type NXT_DSC_ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
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
    pub fn nxt_dsc_add(&mut self) -> NXT_DSC_ADD_W<DEVDMANXTDSC5_SPEC> {
        NXT_DSC_ADD_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdmanxtdsc5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdmanxtdsc5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVDMANXTDSC5_SPEC;
impl crate::RegisterSpec for DEVDMANXTDSC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdmanxtdsc5::R`](R) reader structure"]
impl crate::Readable for DEVDMANXTDSC5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devdmanxtdsc5::W`](W) writer structure"]
impl crate::Writable for DEVDMANXTDSC5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVDMANXTDSC5 to value 0"]
impl crate::Resettable for DEVDMANXTDSC5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
