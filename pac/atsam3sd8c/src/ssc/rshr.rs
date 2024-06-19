#[doc = "Register `RSHR` reader"]
pub type R = crate::R<RshrSpec>;
#[doc = "Field `RSDAT` reader - Receive Synchronization Data"]
pub type RsdatR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Synchronization Data"]
    #[inline(always)]
    pub fn rsdat(&self) -> RsdatR {
        RsdatR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive Sync. Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rshr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RshrSpec;
impl crate::RegisterSpec for RshrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rshr::R`](R) reader structure"]
impl crate::Readable for RshrSpec {}
#[doc = "`reset()` method sets RSHR to value 0"]
impl crate::Resettable for RshrSpec {
    const RESET_VALUE: u32 = 0;
}
