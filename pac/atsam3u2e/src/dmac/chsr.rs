#[doc = "Register `CHSR` reader"]
pub type R = crate::R<ChsrSpec>;
#[doc = "Field `ENA0` reader - Enable \\[3:0\\]"]
pub type Ena0R = crate::BitReader;
#[doc = "Field `ENA1` reader - Enable \\[3:0\\]"]
pub type Ena1R = crate::BitReader;
#[doc = "Field `ENA2` reader - Enable \\[3:0\\]"]
pub type Ena2R = crate::BitReader;
#[doc = "Field `ENA3` reader - Enable \\[3:0\\]"]
pub type Ena3R = crate::BitReader;
#[doc = "Field `SUSP0` reader - Suspend \\[3:0\\]"]
pub type Susp0R = crate::BitReader;
#[doc = "Field `SUSP1` reader - Suspend \\[3:0\\]"]
pub type Susp1R = crate::BitReader;
#[doc = "Field `SUSP2` reader - Suspend \\[3:0\\]"]
pub type Susp2R = crate::BitReader;
#[doc = "Field `SUSP3` reader - Suspend \\[3:0\\]"]
pub type Susp3R = crate::BitReader;
#[doc = "Field `EMPT0` reader - Empty \\[3:0\\]"]
pub type Empt0R = crate::BitReader;
#[doc = "Field `EMPT1` reader - Empty \\[3:0\\]"]
pub type Empt1R = crate::BitReader;
#[doc = "Field `EMPT2` reader - Empty \\[3:0\\]"]
pub type Empt2R = crate::BitReader;
#[doc = "Field `EMPT3` reader - Empty \\[3:0\\]"]
pub type Empt3R = crate::BitReader;
#[doc = "Field `STAL0` reader - Stalled \\[3:0\\]"]
pub type Stal0R = crate::BitReader;
#[doc = "Field `STAL1` reader - Stalled \\[3:0\\]"]
pub type Stal1R = crate::BitReader;
#[doc = "Field `STAL2` reader - Stalled \\[3:0\\]"]
pub type Stal2R = crate::BitReader;
#[doc = "Field `STAL3` reader - Stalled \\[3:0\\]"]
pub type Stal3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable \\[3:0\\]"]
    #[inline(always)]
    pub fn ena0(&self) -> Ena0R {
        Ena0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable \\[3:0\\]"]
    #[inline(always)]
    pub fn ena1(&self) -> Ena1R {
        Ena1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable \\[3:0\\]"]
    #[inline(always)]
    pub fn ena2(&self) -> Ena2R {
        Ena2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable \\[3:0\\]"]
    #[inline(always)]
    pub fn ena3(&self) -> Ena3R {
        Ena3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Suspend \\[3:0\\]"]
    #[inline(always)]
    pub fn susp0(&self) -> Susp0R {
        Susp0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Suspend \\[3:0\\]"]
    #[inline(always)]
    pub fn susp1(&self) -> Susp1R {
        Susp1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Suspend \\[3:0\\]"]
    #[inline(always)]
    pub fn susp2(&self) -> Susp2R {
        Susp2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend \\[3:0\\]"]
    #[inline(always)]
    pub fn susp3(&self) -> Susp3R {
        Susp3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Empty \\[3:0\\]"]
    #[inline(always)]
    pub fn empt0(&self) -> Empt0R {
        Empt0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Empty \\[3:0\\]"]
    #[inline(always)]
    pub fn empt1(&self) -> Empt1R {
        Empt1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Empty \\[3:0\\]"]
    #[inline(always)]
    pub fn empt2(&self) -> Empt2R {
        Empt2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Empty \\[3:0\\]"]
    #[inline(always)]
    pub fn empt3(&self) -> Empt3R {
        Empt3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Stalled \\[3:0\\]"]
    #[inline(always)]
    pub fn stal0(&self) -> Stal0R {
        Stal0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stalled \\[3:0\\]"]
    #[inline(always)]
    pub fn stal1(&self) -> Stal1R {
        Stal1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stalled \\[3:0\\]"]
    #[inline(always)]
    pub fn stal2(&self) -> Stal2R {
        Stal2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stalled \\[3:0\\]"]
    #[inline(always)]
    pub fn stal3(&self) -> Stal3R {
        Stal3R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "DMAC Channel Handler Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChsrSpec;
impl crate::RegisterSpec for ChsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsr::R`](R) reader structure"]
impl crate::Readable for ChsrSpec {}
#[doc = "`reset()` method sets CHSR to value 0x00ff_0000"]
impl crate::Resettable for ChsrSpec {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
