#[doc = "Register `ROV` reader"]
pub type R = crate::R<RovSpec>;
#[doc = "Register `ROV` writer"]
pub type W = crate::W<RovSpec>;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type RovrR = crate::FieldReader;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type RovrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> RovrR {
        RovrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rovr(&mut self) -> RovrW<RovSpec> {
        RovrW::new(self, 0)
    }
}
#[doc = "Receive Overrun Errors Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rov::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rov::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RovSpec;
impl crate::RegisterSpec for RovSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rov::R`](R) reader structure"]
impl crate::Readable for RovSpec {}
#[doc = "`write(|w| ..)` method takes [`rov::W`](W) writer structure"]
impl crate::Writable for RovSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROV to value 0"]
impl crate::Resettable for RovSpec {
    const RESET_VALUE: u32 = 0;
}
