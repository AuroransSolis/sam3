#[doc = "Register `CMPV4` reader"]
pub type R = crate::R<CMPV4_SPEC>;
#[doc = "Register `CMPV4` writer"]
pub type W = crate::W<CMPV4_SPEC>;
#[doc = "Field `CV` reader - Comparison x Value"]
pub type CV_R = crate::FieldReader<u32>;
#[doc = "Field `CV` writer - Comparison x Value"]
pub type CV_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `CVM` reader - Comparison x Value Mode"]
pub type CVM_R = crate::BitReader;
#[doc = "Field `CVM` writer - Comparison x Value Mode"]
pub type CVM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&self) -> CVM_R {
        CVM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    #[must_use]
    pub fn cv(&mut self) -> CV_W<CMPV4_SPEC> {
        CV_W::new(self, 0)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cvm(&mut self) -> CVM_W<CMPV4_SPEC> {
        CVM_W::new(self, 24)
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
#[doc = "PWM Comparison 4 Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpv4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpv4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPV4_SPEC;
impl crate::RegisterSpec for CMPV4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpv4::R`](R) reader structure"]
impl crate::Readable for CMPV4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpv4::W`](W) writer structure"]
impl crate::Writable for CMPV4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPV4 to value 0"]
impl crate::Resettable for CMPV4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
