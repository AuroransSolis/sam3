#[doc = "Register `WPSR` reader"]
pub type R = crate::R<WpsrSpec>;
#[doc = "Field `WPROTERR` reader - Write PROTection ERRor"]
pub type WproterrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write PROTection ERRor"]
    #[inline(always)]
    pub fn wproterr(&self) -> WproterrR {
        WproterrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Write Protect Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsrSpec;
impl crate::RegisterSpec for WpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsr::R`](R) reader structure"]
impl crate::Readable for WpsrSpec {}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WpsrSpec {
    const RESET_VALUE: u32 = 0;
}
