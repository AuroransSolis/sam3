#[doc = "Register `CDTY3` reader"]
pub type R = crate::R<CDTY3_SPEC>;
#[doc = "Register `CDTY3` writer"]
pub type W = crate::W<CDTY3_SPEC>;
#[doc = "Field `CDTY` reader - Channel Duty Cycle"]
pub type CDTY_R = crate::FieldReader<u32>;
#[doc = "Field `CDTY` writer - Channel Duty Cycle"]
pub type CDTY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Duty Cycle"]
    #[inline(always)]
    pub fn cdty(&self) -> CDTY_R {
        CDTY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Duty Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cdty(&mut self) -> CDTY_W<CDTY3_SPEC> {
        CDTY_W::new(self, 0)
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
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDTY3_SPEC;
impl crate::RegisterSpec for CDTY3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdty3::R`](R) reader structure"]
impl crate::Readable for CDTY3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdty3::W`](W) writer structure"]
impl crate::Writable for CDTY3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDTY3 to value 0"]
impl crate::Resettable for CDTY3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
