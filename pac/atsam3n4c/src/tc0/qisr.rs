#[doc = "Register `QISR` reader"]
pub type R = crate::R<QisrSpec>;
#[doc = "Field `IDX` reader - Index"]
pub type IdxR = crate::BitReader;
#[doc = "Field `DIRCHG` reader - Direction Change"]
pub type DirchgR = crate::BitReader;
#[doc = "Field `QERR` reader - Quadrature Error"]
pub type QerrR = crate::BitReader;
#[doc = "Field `DIR` reader - Direction"]
pub type DirR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    pub fn idx(&self) -> IdxR {
        IdxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    pub fn dirchg(&self) -> DirchgR {
        DirchgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    pub fn qerr(&self) -> QerrR {
        QerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "QDEC Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QisrSpec;
impl crate::RegisterSpec for QisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qisr::R`](R) reader structure"]
impl crate::Readable for QisrSpec {}
#[doc = "`reset()` method sets QISR to value 0"]
impl crate::Resettable for QisrSpec {
    const RESET_VALUE: u32 = 0;
}
