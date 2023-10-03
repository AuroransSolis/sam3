#[doc = "Register `ELE` reader"]
pub type R = crate::R<ELE_SPEC>;
#[doc = "Register `ELE` writer"]
pub type W = crate::W<ELE_SPEC>;
#[doc = "Field `EXL` reader - Excessive Length Errors"]
pub type EXL_R = crate::FieldReader;
#[doc = "Field `EXL` writer - Excessive Length Errors"]
pub type EXL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Excessive Length Errors"]
    #[inline(always)]
    pub fn exl(&self) -> EXL_R {
        EXL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Excessive Length Errors"]
    #[inline(always)]
    #[must_use]
    pub fn exl(&mut self) -> EXL_W<ELE_SPEC, 0> {
        EXL_W::new(self)
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
#[doc = "Excessive Length Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ele::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ele::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ELE_SPEC;
impl crate::RegisterSpec for ELE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ele::R`](R) reader structure"]
impl crate::Readable for ELE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ele::W`](W) writer structure"]
impl crate::Writable for ELE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ELE to value 0"]
impl crate::Resettable for ELE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
