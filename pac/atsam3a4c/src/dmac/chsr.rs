#[doc = "Register `CHSR` reader"]
pub type R = crate::R<CHSR_SPEC>;
#[doc = "Field `ENA0` reader - Enable \\[5:0\\]"]
pub type ENA0_R = crate::BitReader;
#[doc = "Field `ENA1` reader - Enable \\[5:0\\]"]
pub type ENA1_R = crate::BitReader;
#[doc = "Field `ENA2` reader - Enable \\[5:0\\]"]
pub type ENA2_R = crate::BitReader;
#[doc = "Field `ENA3` reader - Enable \\[5:0\\]"]
pub type ENA3_R = crate::BitReader;
#[doc = "Field `ENA4` reader - Enable \\[5:0\\]"]
pub type ENA4_R = crate::BitReader;
#[doc = "Field `ENA5` reader - Enable \\[5:0\\]"]
pub type ENA5_R = crate::BitReader;
#[doc = "Field `SUSP0` reader - Suspend \\[5:0\\]"]
pub type SUSP0_R = crate::BitReader;
#[doc = "Field `SUSP1` reader - Suspend \\[5:0\\]"]
pub type SUSP1_R = crate::BitReader;
#[doc = "Field `SUSP2` reader - Suspend \\[5:0\\]"]
pub type SUSP2_R = crate::BitReader;
#[doc = "Field `SUSP3` reader - Suspend \\[5:0\\]"]
pub type SUSP3_R = crate::BitReader;
#[doc = "Field `SUSP4` reader - Suspend \\[5:0\\]"]
pub type SUSP4_R = crate::BitReader;
#[doc = "Field `SUSP5` reader - Suspend \\[5:0\\]"]
pub type SUSP5_R = crate::BitReader;
#[doc = "Field `EMPT0` reader - Empty \\[5:0\\]"]
pub type EMPT0_R = crate::BitReader;
#[doc = "Field `EMPT1` reader - Empty \\[5:0\\]"]
pub type EMPT1_R = crate::BitReader;
#[doc = "Field `EMPT2` reader - Empty \\[5:0\\]"]
pub type EMPT2_R = crate::BitReader;
#[doc = "Field `EMPT3` reader - Empty \\[5:0\\]"]
pub type EMPT3_R = crate::BitReader;
#[doc = "Field `EMPT4` reader - Empty \\[5:0\\]"]
pub type EMPT4_R = crate::BitReader;
#[doc = "Field `EMPT5` reader - Empty \\[5:0\\]"]
pub type EMPT5_R = crate::BitReader;
#[doc = "Field `STAL0` reader - Stalled \\[5:0\\]"]
pub type STAL0_R = crate::BitReader;
#[doc = "Field `STAL1` reader - Stalled \\[5:0\\]"]
pub type STAL1_R = crate::BitReader;
#[doc = "Field `STAL2` reader - Stalled \\[5:0\\]"]
pub type STAL2_R = crate::BitReader;
#[doc = "Field `STAL3` reader - Stalled \\[5:0\\]"]
pub type STAL3_R = crate::BitReader;
#[doc = "Field `STAL4` reader - Stalled \\[5:0\\]"]
pub type STAL4_R = crate::BitReader;
#[doc = "Field `STAL5` reader - Stalled \\[5:0\\]"]
pub type STAL5_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena0(&self) -> ENA0_R {
        ENA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena1(&self) -> ENA1_R {
        ENA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena2(&self) -> ENA2_R {
        ENA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena3(&self) -> ENA3_R {
        ENA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena4(&self) -> ENA4_R {
        ENA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena5(&self) -> ENA5_R {
        ENA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp0(&self) -> SUSP0_R {
        SUSP0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp1(&self) -> SUSP1_R {
        SUSP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp2(&self) -> SUSP2_R {
        SUSP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp3(&self) -> SUSP3_R {
        SUSP3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp4(&self) -> SUSP4_R {
        SUSP4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp5(&self) -> SUSP5_R {
        SUSP5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt0(&self) -> EMPT0_R {
        EMPT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt1(&self) -> EMPT1_R {
        EMPT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt2(&self) -> EMPT2_R {
        EMPT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt3(&self) -> EMPT3_R {
        EMPT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt4(&self) -> EMPT4_R {
        EMPT4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt5(&self) -> EMPT5_R {
        EMPT5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal0(&self) -> STAL0_R {
        STAL0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal1(&self) -> STAL1_R {
        STAL1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal2(&self) -> STAL2_R {
        STAL2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal3(&self) -> STAL3_R {
        STAL3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal4(&self) -> STAL4_R {
        STAL4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal5(&self) -> STAL5_R {
        STAL5_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "DMAC Channel Handler Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHSR_SPEC;
impl crate::RegisterSpec for CHSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsr::R`](R) reader structure"]
impl crate::Readable for CHSR_SPEC {}
#[doc = "`reset()` method sets CHSR to value 0x00ff_0000"]
impl crate::Resettable for CHSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_0000;
}
