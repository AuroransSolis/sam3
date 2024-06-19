#[doc = "Register `RRE` reader"]
pub type R = crate::R<RreSpec>;
#[doc = "Register `RRE` writer"]
pub type W = crate::W<RreSpec>;
#[doc = "Field `RRE` reader - Receive Resource Errors"]
pub type RreR = crate::FieldReader<u16>;
#[doc = "Field `RRE` writer - Receive Resource Errors"]
pub type RreW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Resource Errors"]
    #[inline(always)]
    pub fn rre(&self) -> RreR {
        RreR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Resource Errors"]
    #[inline(always)]
    #[must_use]
    pub fn rre(&mut self) -> RreW<RreSpec> {
        RreW::new(self, 0)
    }
}
#[doc = "Receive Resource Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RreSpec;
impl crate::RegisterSpec for RreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rre::R`](R) reader structure"]
impl crate::Readable for RreSpec {}
#[doc = "`write(|w| ..)` method takes [`rre::W`](W) writer structure"]
impl crate::Writable for RreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RRE to value 0"]
impl crate::Resettable for RreSpec {
    const RESET_VALUE: u32 = 0;
}
