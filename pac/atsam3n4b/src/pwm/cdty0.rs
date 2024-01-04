#[doc = "Register `CDTY0` reader"]
pub type R = crate::R<CDTY0_SPEC>;
#[doc = "Register `CDTY0` writer"]
pub type W = crate::W<CDTY0_SPEC>;
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
    pub fn cdty(&mut self) -> CDTY_W<CDTY0_SPEC> {
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
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdty0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdty0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDTY0_SPEC;
impl crate::RegisterSpec for CDTY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdty0::R`](R) reader structure"]
impl crate::Readable for CDTY0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdty0::W`](W) writer structure"]
impl crate::Writable for CDTY0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDTY0 to value 0"]
impl crate::Resettable for CDTY0_SPEC {
    const RESET_VALUE: u32 = 0;
}
