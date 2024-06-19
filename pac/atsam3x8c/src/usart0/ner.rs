#[doc = "Register `NER` reader"]
pub type R = crate::R<NerSpec>;
#[doc = "Field `NB_ERRORS` reader - Number of Errors"]
pub type NbErrorsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of Errors"]
    #[inline(always)]
    pub fn nb_errors(&self) -> NbErrorsR {
        NbErrorsR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ner::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NerSpec;
impl crate::RegisterSpec for NerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ner::R`](R) reader structure"]
impl crate::Readable for NerSpec {}
