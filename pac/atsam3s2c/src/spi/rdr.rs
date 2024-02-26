#[doc = "Register `RDR` reader"]
pub type R = crate::R<RdrSpec>;
#[doc = "Field `RD` reader - Receive Data"]
pub type RdR = crate::FieldReader<u16>;
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub type PcsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PcsR {
        PcsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Receive Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrSpec;
impl crate::RegisterSpec for RdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RdrSpec {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RdrSpec {
    const RESET_VALUE: u32 = 0;
}
