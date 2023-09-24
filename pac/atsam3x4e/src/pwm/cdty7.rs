#[doc = "Register `CDTY7` reader"]
pub type R = crate::R<CDTY7_SPEC>;
#[doc = "Register `CDTY7` writer"]
pub type W = crate::W<CDTY7_SPEC>;
#[doc = "Field `CDTY` reader - Channel Duty-Cycle"]
pub type CDTY_R = crate::FieldReader<u32>;
#[doc = "Field `CDTY` writer - Channel Duty-Cycle"]
pub type CDTY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&self) -> CDTY_R {
        CDTY_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cdty(&mut self) -> CDTY_W<CDTY7_SPEC, 0> {
        CDTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDTY7_SPEC;
impl crate::RegisterSpec for CDTY7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdty7::R`](R) reader structure"]
impl crate::Readable for CDTY7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdty7::W`](W) writer structure"]
impl crate::Writable for CDTY7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDTY7 to value 0"]
impl crate::Resettable for CDTY7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
