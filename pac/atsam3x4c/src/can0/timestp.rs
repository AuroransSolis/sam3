#[doc = "Register `TIMESTP` reader"]
pub type R = crate::R<TimestpSpec>;
#[doc = "Field `MTIMESTAMP` reader - Timestamp"]
pub type MtimestampR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp"]
    #[inline(always)]
    pub fn mtimestamp(&self) -> MtimestampR {
        MtimestampR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timestamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestpSpec;
impl crate::RegisterSpec for TimestpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestp::R`](R) reader structure"]
impl crate::Readable for TimestpSpec {}
#[doc = "`reset()` method sets TIMESTP to value 0"]
impl crate::Resettable for TimestpSpec {
    const RESET_VALUE: u32 = 0;
}
