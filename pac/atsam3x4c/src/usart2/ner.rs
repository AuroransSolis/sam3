#[doc = "Register `NER` reader"]
pub struct R(crate::R<NER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NB_ERRORS` reader - Number of Errors"]
pub type NB_ERRORS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of Errors"]
    #[inline(always)]
    pub fn nb_errors(&self) -> NB_ERRORS_R {
        NB_ERRORS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ner](index.html) module"]
pub struct NER_SPEC;
impl crate::RegisterSpec for NER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ner::R](R) reader structure"]
impl crate::Readable for NER_SPEC {
    type Reader = R;
}
