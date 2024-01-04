#[doc = "Register `CPRD3` reader"]
pub type R = crate::R<CPRD3_SPEC>;
#[doc = "Register `CPRD3` writer"]
pub type W = crate::W<CPRD3_SPEC>;
#[doc = "Field `CPRD` reader - Channel Period"]
pub type CPRD_R = crate::FieldReader<u32>;
#[doc = "Field `CPRD` writer - Channel Period"]
pub type CPRD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CPRD_R {
        CPRD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    #[must_use]
    pub fn cprd(&mut self) -> CPRD_W<CPRD3_SPEC> {
        CPRD_W::new(self, 0)
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
#[doc = "PWM Channel Period Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cprd3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cprd3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPRD3_SPEC;
impl crate::RegisterSpec for CPRD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cprd3::R`](R) reader structure"]
impl crate::Readable for CPRD3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cprd3::W`](W) writer structure"]
impl crate::Writable for CPRD3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPRD3 to value 0"]
impl crate::Resettable for CPRD3_SPEC {
    const RESET_VALUE: u32 = 0;
}
