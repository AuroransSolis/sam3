#[doc = "Register `HSTFNUM` reader"]
pub type R = crate::R<HstfnumSpec>;
#[doc = "Register `HSTFNUM` writer"]
pub type W = crate::W<HstfnumSpec>;
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub type MfnumR = crate::FieldReader;
#[doc = "Field `MFNUM` writer - Micro Frame Number"]
pub type MfnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FNUM` reader - Frame Number"]
pub type FnumR = crate::FieldReader<u16>;
#[doc = "Field `FNUM` writer - Frame Number"]
pub type FnumW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `FLENHIGH` reader - Frame Length"]
pub type FlenhighR = crate::FieldReader;
#[doc = "Field `FLENHIGH` writer - Frame Length"]
pub type FlenhighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MfnumR {
        MfnumR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FnumR {
        FnumR::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&self) -> FlenhighR {
        FlenhighR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn mfnum(&mut self) -> MfnumW<HstfnumSpec> {
        MfnumW::new(self, 0)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    #[must_use]
    pub fn fnum(&mut self) -> FnumW<HstfnumSpec> {
        FnumW::new(self, 3)
    }
    #[doc = "Bits 16:23 - Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn flenhigh(&mut self) -> FlenhighW<HstfnumSpec> {
        FlenhighW::new(self, 16)
    }
}
#[doc = "Host Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstfnum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstfnum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstfnumSpec;
impl crate::RegisterSpec for HstfnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstfnum::R`](R) reader structure"]
impl crate::Readable for HstfnumSpec {}
#[doc = "`write(|w| ..)` method takes [`hstfnum::W`](W) writer structure"]
impl crate::Writable for HstfnumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSTFNUM to value 0"]
impl crate::Resettable for HstfnumSpec {
    const RESET_VALUE: u32 = 0;
}
