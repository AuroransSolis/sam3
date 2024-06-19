#[doc = "Register `CKGR_UCKR` reader"]
pub type R = crate::R<CkgrUckrSpec>;
#[doc = "Register `CKGR_UCKR` writer"]
pub type W = crate::W<CkgrUckrSpec>;
#[doc = "Field `UPLLEN` reader - UTMI PLL Enable"]
pub type UpllenR = crate::BitReader;
#[doc = "Field `UPLLEN` writer - UTMI PLL Enable"]
pub type UpllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPLLCOUNT` reader - UTMI PLL Start-up Time"]
pub type UpllcountR = crate::FieldReader;
#[doc = "Field `UPLLCOUNT` writer - UTMI PLL Start-up Time"]
pub type UpllcountW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 16 - UTMI PLL Enable"]
    #[inline(always)]
    pub fn upllen(&self) -> UpllenR {
        UpllenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - UTMI PLL Start-up Time"]
    #[inline(always)]
    pub fn upllcount(&self) -> UpllcountR {
        UpllcountR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - UTMI PLL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn upllen(&mut self) -> UpllenW<CkgrUckrSpec> {
        UpllenW::new(self, 16)
    }
    #[doc = "Bits 20:23 - UTMI PLL Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn upllcount(&mut self) -> UpllcountW<CkgrUckrSpec> {
        UpllcountW::new(self, 20)
    }
}
#[doc = "UTMI Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_uckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_uckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgrUckrSpec;
impl crate::RegisterSpec for CkgrUckrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_uckr::R`](R) reader structure"]
impl crate::Readable for CkgrUckrSpec {}
#[doc = "`write(|w| ..)` method takes [`ckgr_uckr::W`](W) writer structure"]
impl crate::Writable for CkgrUckrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKGR_UCKR to value 0x1020_0800"]
impl crate::Resettable for CkgrUckrSpec {
    const RESET_VALUE: u32 = 0x1020_0800;
}
