#[doc = "Register `TID` reader"]
pub type R = crate::R<TidSpec>;
#[doc = "Register `TID` writer"]
pub type W = crate::W<TidSpec>;
#[doc = "Field `TID` reader - Type ID checking"]
pub type TidR = crate::FieldReader<u16>;
#[doc = "Field `TID` writer - Type ID checking"]
pub type TidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Type ID checking"]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID checking"]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TidW<TidSpec> {
        TidW::new(self, 0)
    }
}
#[doc = "Type ID Checking Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TidSpec;
impl crate::RegisterSpec for TidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tid::R`](R) reader structure"]
impl crate::Readable for TidSpec {}
#[doc = "`write(|w| ..)` method takes [`tid::W`](W) writer structure"]
impl crate::Writable for TidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TID to value 0"]
impl crate::Resettable for TidSpec {
    const RESET_VALUE: u32 = 0;
}
