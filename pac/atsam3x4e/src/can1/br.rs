#[doc = "Register `BR` reader"]
pub type R = crate::R<BrSpec>;
#[doc = "Register `BR` writer"]
pub type W = crate::W<BrSpec>;
#[doc = "Field `PHASE2` reader - Phase 2 segment"]
pub type Phase2R = crate::FieldReader;
#[doc = "Field `PHASE2` writer - Phase 2 segment"]
pub type Phase2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHASE1` reader - Phase 1 segment"]
pub type Phase1R = crate::FieldReader;
#[doc = "Field `PHASE1` writer - Phase 1 segment"]
pub type Phase1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PROPAG` reader - Programming time segment"]
pub type PropagR = crate::FieldReader;
#[doc = "Field `PROPAG` writer - Programming time segment"]
pub type PropagW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SJW` reader - Re-synchronization jump width"]
pub type SjwR = crate::FieldReader;
#[doc = "Field `SJW` writer - Re-synchronization jump width"]
pub type SjwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BRP` reader - Baudrate Prescaler."]
pub type BrpR = crate::FieldReader;
#[doc = "Field `BRP` writer - Baudrate Prescaler."]
pub type BrpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Sampling Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smp {
    #[doc = "0: The incoming bit stream is sampled once at sample point."]
    Once = 0,
    #[doc = "1: The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    Three = 1,
}
impl From<Smp> for bool {
    #[inline(always)]
    fn from(variant: Smp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMP` reader - Sampling Mode"]
pub type SmpR = crate::BitReader<Smp>;
impl SmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smp {
        match self.bits {
            false => Smp::Once,
            true => Smp::Three,
        }
    }
    #[doc = "The incoming bit stream is sampled once at sample point."]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == Smp::Once
    }
    #[doc = "The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Smp::Three
    }
}
#[doc = "Field `SMP` writer - Sampling Mode"]
pub type SmpW<'a, REG> = crate::BitWriter<'a, REG, Smp>;
impl<'a, REG> SmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The incoming bit stream is sampled once at sample point."]
    #[inline(always)]
    pub fn once(self) -> &'a mut crate::W<REG> {
        self.variant(Smp::Once)
    }
    #[doc = "The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Smp::Three)
    }
}
impl R {
    #[doc = "Bits 0:2 - Phase 2 segment"]
    #[inline(always)]
    pub fn phase2(&self) -> Phase2R {
        Phase2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Phase 1 segment"]
    #[inline(always)]
    pub fn phase1(&self) -> Phase1R {
        Phase1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Programming time segment"]
    #[inline(always)]
    pub fn propag(&self) -> PropagR {
        PropagR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Re-synchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SjwR {
        SjwR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Baudrate Prescaler."]
    #[inline(always)]
    pub fn brp(&self) -> BrpR {
        BrpR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Sampling Mode"]
    #[inline(always)]
    pub fn smp(&self) -> SmpR {
        SmpR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Phase 2 segment"]
    #[inline(always)]
    #[must_use]
    pub fn phase2(&mut self) -> Phase2W<BrSpec> {
        Phase2W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Phase 1 segment"]
    #[inline(always)]
    #[must_use]
    pub fn phase1(&mut self) -> Phase1W<BrSpec> {
        Phase1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Programming time segment"]
    #[inline(always)]
    #[must_use]
    pub fn propag(&mut self) -> PropagW<BrSpec> {
        PropagW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Re-synchronization jump width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SjwW<BrSpec> {
        SjwW::new(self, 12)
    }
    #[doc = "Bits 16:22 - Baudrate Prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BrpW<BrSpec> {
        BrpW::new(self, 16)
    }
    #[doc = "Bit 24 - Sampling Mode"]
    #[inline(always)]
    #[must_use]
    pub fn smp(&mut self) -> SmpW<BrSpec> {
        SmpW::new(self, 24)
    }
}
#[doc = "Baudrate Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrSpec;
impl crate::RegisterSpec for BrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`br::R`](R) reader structure"]
impl crate::Readable for BrSpec {}
#[doc = "`write(|w| ..)` method takes [`br::W`](W) writer structure"]
impl crate::Writable for BrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BR to value 0"]
impl crate::Resettable for BrSpec {
    const RESET_VALUE: u32 = 0;
}
