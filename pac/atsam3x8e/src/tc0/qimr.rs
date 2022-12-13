#[doc = "Register `QIMR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<QIMR_SPEC>);
#[doc = "Field `IDX` reader - InDeX"]
pub type IDX_R = crate::BitReader<bool>;
#[doc = "Field `DIRCHG` reader - DIRection CHanGe"]
pub type DIRCHG_R = crate::BitReader<bool>;
#[doc = "Field `QERR` reader - Quadrature ERRor"]
pub type QERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - InDeX"]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIRection CHanGe"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Quadrature ERRor"]
    #[inline(always)]
    pub fn qerr(&self) -> QERR_R {
        QERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "QDEC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qimr](index.html) module"]
pub struct QIMR_SPEC;
impl crate::RegisterSpec for QIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qimr::R](R) reader structure"]
impl crate::Readable for QIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QIMR to value 0"]
impl crate::Resettable for QIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
