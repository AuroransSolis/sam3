#[doc = "Register `PCIMR` reader"]
pub type R = crate::R<PcimrSpec>;
#[doc = "Field `DRDY` reader - Parallel Capture Mode Data Ready Interrupt Mask"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `OVRE` reader - Parallel Capture Mode Overrun Error Interrupt Mask"]
pub type OvreR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of Reception Transfer Interrupt Mask"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Reception Buffer Full Interrupt Mask"]
pub type RxbuffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Reception Transfer Interrupt Mask"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reception Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Parallel Capture Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcimrSpec;
impl crate::RegisterSpec for PcimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcimr::R`](R) reader structure"]
impl crate::Readable for PcimrSpec {}
#[doc = "`reset()` method sets PCIMR to value 0"]
impl crate::Resettable for PcimrSpec {
    const RESET_VALUE: u32 = 0;
}
