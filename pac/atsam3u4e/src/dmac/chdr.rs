#[doc = "Register `CHDR` writer"]
pub type W = crate::W<ChdrSpec>;
#[doc = "Field `DIS0` writer - Disable \\[3:0\\]"]
pub type Dis0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS1` writer - Disable \\[3:0\\]"]
pub type Dis1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS2` writer - Disable \\[3:0\\]"]
pub type Dis2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS3` writer - Disable \\[3:0\\]"]
pub type Dis3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0` writer - Resume \\[3:0\\]"]
pub type Res0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES1` writer - Resume \\[3:0\\]"]
pub type Res1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES2` writer - Resume \\[3:0\\]"]
pub type Res2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES3` writer - Resume \\[3:0\\]"]
pub type Res3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis0(&mut self) -> Dis0W<ChdrSpec> {
        Dis0W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis1(&mut self) -> Dis1W<ChdrSpec> {
        Dis1W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis2(&mut self) -> Dis2W<ChdrSpec> {
        Dis2W::new(self, 2)
    }
    #[doc = "Bit 3 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis3(&mut self) -> Dis3W<ChdrSpec> {
        Dis3W::new(self, 3)
    }
    #[doc = "Bit 8 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res0(&mut self) -> Res0W<ChdrSpec> {
        Res0W::new(self, 8)
    }
    #[doc = "Bit 9 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res1(&mut self) -> Res1W<ChdrSpec> {
        Res1W::new(self, 9)
    }
    #[doc = "Bit 10 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res2(&mut self) -> Res2W<ChdrSpec> {
        Res2W::new(self, 10)
    }
    #[doc = "Bit 11 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res3(&mut self) -> Res3W<ChdrSpec> {
        Res3W::new(self, 11)
    }
}
#[doc = "DMAC Channel Handler Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChdrSpec;
impl crate::RegisterSpec for ChdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chdr::W`](W) writer structure"]
impl crate::Writable for ChdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
