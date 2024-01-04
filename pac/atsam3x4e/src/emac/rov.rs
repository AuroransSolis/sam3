#[doc = "Register `ROV` reader"]
pub type R = crate::R<ROV_SPEC>;
#[doc = "Register `ROV` writer"]
pub type W = crate::W<ROV_SPEC>;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type ROVR_R = crate::FieldReader;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type ROVR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> ROVR_W<ROV_SPEC> {
        ROVR_W::new(self, 0)
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
#[doc = "Receive Overrun Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rov::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rov::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROV_SPEC;
impl crate::RegisterSpec for ROV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rov::R`](R) reader structure"]
impl crate::Readable for ROV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rov::W`](W) writer structure"]
impl crate::Writable for ROV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROV to value 0"]
impl crate::Resettable for ROV_SPEC {
    const RESET_VALUE: u32 = 0;
}
