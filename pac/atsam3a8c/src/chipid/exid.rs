#[doc = "Register `EXID` reader"]
pub type R = crate::R<ExidSpec>;
#[doc = "Field `EXID` reader - Chip ID Extension"]
pub type ExidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Chip ID Extension"]
    #[inline(always)]
    pub fn exid(&self) -> ExidR {
        ExidR::new(self.bits)
    }
}
#[doc = "Chip ID Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExidSpec;
impl crate::RegisterSpec for ExidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exid::R`](R) reader structure"]
impl crate::Readable for ExidSpec {}
