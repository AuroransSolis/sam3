#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `CE` reader - Comparison Edge"]
pub type CeR = crate::BitReader;
#[doc = "Field `SCO` reader - Synchronized Comparator Output"]
pub type ScoR = crate::BitReader;
#[doc = "Field `MASK` reader - "]
pub type MaskR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Comparison Edge"]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronized Comparator Output"]
    #[inline(always)]
    pub fn sco(&self) -> ScoR {
        ScoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
