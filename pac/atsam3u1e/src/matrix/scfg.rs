#[doc = "Register `SCFG[%s]` reader"]
pub struct R(crate::R<SCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFG[%s]` writer"]
pub struct W(crate::W<SCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOT_CYCLE` reader - Maximum Number of Allowed Cycles for a Burst"]
pub type SLOT_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLOT_CYCLE` writer - Maximum Number of Allowed Cycles for a Burst"]
pub type SLOT_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub type DEFMSTR_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub type DEFMSTR_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Default Master"]
pub type FIXED_DEFMSTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Default Master"]
pub type FIXED_DEFMSTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `ARBT` reader - Arbitration Type"]
pub type ARBT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARBT` writer - Arbitration Type"]
pub type ARBT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SLOT_CYCLE_R {
        SLOT_CYCLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPE_R {
        DEFMSTR_TYPE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTR_R {
        FIXED_DEFMSTR_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&self) -> ARBT_R {
        ARBT_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&mut self) -> SLOT_CYCLE_W<0> {
        SLOT_CYCLE_W::new(self)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&mut self) -> DEFMSTR_TYPE_W<16> {
        DEFMSTR_TYPE_W::new(self)
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&mut self) -> FIXED_DEFMSTR_W<18> {
        FIXED_DEFMSTR_W::new(self)
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&mut self) -> ARBT_W<24> {
        ARBT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfg](index.html) module"]
pub struct SCFG_SPEC;
impl crate::RegisterSpec for SCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfg::R](R) reader structure"]
impl crate::Readable for SCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfg::W](W) writer structure"]
impl crate::Writable for SCFG_SPEC {
    type Writer = W;
}
