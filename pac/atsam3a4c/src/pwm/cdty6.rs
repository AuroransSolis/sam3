#[doc = "Register `CDTY6` reader"]
pub type R = crate::R<CDTY6_SPEC>;
#[doc = "Register `CDTY6` writer"]
pub type W = crate::W<CDTY6_SPEC>;
#[doc = "Field `CDTY` reader - Channel Duty-Cycle"]
pub type CDTY_R = crate::FieldReader<u32>;
#[doc = "Field `CDTY` writer - Channel Duty-Cycle"]
pub type CDTY_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
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
    pub fn cdty(&mut self) -> CDTY_W<CDTY6_SPEC> {
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
#[doc = "PWM Channel Duty Cycle Register (ch_num = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDTY6_SPEC;
impl crate::RegisterSpec for CDTY6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdty6::R`](R) reader structure"]
impl crate::Readable for CDTY6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdty6::W`](W) writer structure"]
impl crate::Writable for CDTY6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDTY6 to value 0"]
impl crate::Resettable for CDTY6_SPEC {
    const RESET_VALUE: u32 = 0;
}
