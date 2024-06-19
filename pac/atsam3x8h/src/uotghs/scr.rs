#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `IDTIC` writer - ID Transition Interrupt Clear"]
pub type IdticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSTIC` writer - VBus Transition Interrupt Clear"]
pub type VbusticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPIC` writer - SRP Interrupt Clear"]
pub type SrpicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBERRIC` writer - VBus Error Interrupt Clear"]
pub type VberricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCERRIC` writer - B-Connection Error Interrupt Clear"]
pub type BcerricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROLEEXIC` writer - Role Exchange Interrupt Clear"]
pub type RoleexicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPERRIC` writer - HNP Error Interrupt Clear"]
pub type HnperricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOIC` writer - Suspend Time-Out Interrupt Clear"]
pub type StoicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSRQC` writer - VBus Request Clear"]
pub type VbusrqcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn idtic(&mut self) -> IdticW<ScrSpec> {
        IdticW::new(self, 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbustic(&mut self) -> VbusticW<ScrSpec> {
        VbusticW::new(self, 1)
    }
    #[doc = "Bit 2 - SRP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn srpic(&mut self) -> SrpicW<ScrSpec> {
        SrpicW::new(self, 2)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vberric(&mut self) -> VberricW<ScrSpec> {
        VberricW::new(self, 3)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bcerric(&mut self) -> BcerricW<ScrSpec> {
        BcerricW::new(self, 4)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn roleexic(&mut self) -> RoleexicW<ScrSpec> {
        RoleexicW::new(self, 5)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hnperric(&mut self) -> HnperricW<ScrSpec> {
        HnperricW::new(self, 6)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stoic(&mut self) -> StoicW<ScrSpec> {
        StoicW::new(self, 7)
    }
    #[doc = "Bit 9 - VBus Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbusrqc(&mut self) -> VbusrqcW<ScrSpec> {
        VbusrqcW::new(self, 9)
    }
}
#[doc = "General Status Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
