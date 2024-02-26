#[doc = "Register `CMR` reader"]
pub type R = crate::R<CmrSpec>;
#[doc = "Register `CMR` writer"]
pub type W = crate::W<CmrSpec>;
#[doc = "Field `DIV` reader - Clock Divider"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Clock Divider"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<CmrSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "Clock Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmrSpec;
impl crate::RegisterSpec for CmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmr::R`](R) reader structure"]
impl crate::Readable for CmrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmr::W`](W) writer structure"]
impl crate::Writable for CmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CmrSpec {
    const RESET_VALUE: u32 = 0;
}
