#[doc = "Register `CHER` writer"]
pub type W = crate::W<CherSpec>;
#[doc = "Field `ENA0` writer - Enable \\[3:0\\]"]
pub type Ena0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA1` writer - Enable \\[3:0\\]"]
pub type Ena1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA2` writer - Enable \\[3:0\\]"]
pub type Ena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENA3` writer - Enable \\[3:0\\]"]
pub type Ena3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP0` writer - Suspend \\[3:0\\]"]
pub type Susp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP1` writer - Suspend \\[3:0\\]"]
pub type Susp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP2` writer - Suspend \\[3:0\\]"]
pub type Susp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP3` writer - Suspend \\[3:0\\]"]
pub type Susp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEP0` writer - Keep on \\[3:0\\]"]
pub type Keep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEP1` writer - Keep on \\[3:0\\]"]
pub type Keep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEP2` writer - Keep on \\[3:0\\]"]
pub type Keep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEP3` writer - Keep on \\[3:0\\]"]
pub type Keep3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Enable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ena0(&mut self) -> Ena0W<CherSpec> {
        Ena0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ena1(&mut self) -> Ena1W<CherSpec> {
        Ena1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ena2(&mut self) -> Ena2W<CherSpec> {
        Ena2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ena3(&mut self) -> Ena3W<CherSpec> {
        Ena3W::new(self, 3)
    }
    #[doc = "Bit 8 - Suspend \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn susp0(&mut self) -> Susp0W<CherSpec> {
        Susp0W::new(self, 8)
    }
    #[doc = "Bit 9 - Suspend \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn susp1(&mut self) -> Susp1W<CherSpec> {
        Susp1W::new(self, 9)
    }
    #[doc = "Bit 10 - Suspend \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn susp2(&mut self) -> Susp2W<CherSpec> {
        Susp2W::new(self, 10)
    }
    #[doc = "Bit 11 - Suspend \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn susp3(&mut self) -> Susp3W<CherSpec> {
        Susp3W::new(self, 11)
    }
    #[doc = "Bit 24 - Keep on \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn keep0(&mut self) -> Keep0W<CherSpec> {
        Keep0W::new(self, 24)
    }
    #[doc = "Bit 25 - Keep on \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn keep1(&mut self) -> Keep1W<CherSpec> {
        Keep1W::new(self, 25)
    }
    #[doc = "Bit 26 - Keep on \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn keep2(&mut self) -> Keep2W<CherSpec> {
        Keep2W::new(self, 26)
    }
    #[doc = "Bit 27 - Keep on \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn keep3(&mut self) -> Keep3W<CherSpec> {
        Keep3W::new(self, 27)
    }
}
#[doc = "DMAC Channel Handler Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cher::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CherSpec;
impl crate::RegisterSpec for CherSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cher::W`](W) writer structure"]
impl crate::Writable for CherSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
