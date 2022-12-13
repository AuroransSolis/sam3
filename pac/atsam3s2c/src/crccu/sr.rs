#[doc = "Register `SR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<SR_SPEC>);
#[doc = "Field `CRC` reader - Cyclic Redundancy Check Value"]
pub type CRC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cyclic Redundancy Check Value"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(self.bits)
    }
}
#[doc = "CRCCU Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0xffff_ffff"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
