#[doc = "Register `TIMINGS3` reader"]
pub struct R(crate::R<TIMINGS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMINGS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMINGS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMINGS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMINGS3` writer"]
pub struct W(crate::W<TIMINGS3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMINGS3_SPEC>;
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
impl From<crate::W<TIMINGS3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMINGS3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCLR` reader - CLE to REN Low Delay"]
pub type TCLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCLR` writer - CLE to REN Low Delay"]
pub type TCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `TADL` reader - ALE to Data Start"]
pub type TADL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TADL` writer - ALE to Data Start"]
pub type TADL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `TAR` reader - ALE to REN Low Delay"]
pub type TAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAR` writer - ALE to REN Low Delay"]
pub type TAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `OCMS` reader - Off Chip Memory Scrambling Enable"]
pub type OCMS_R = crate::BitReader<bool>;
#[doc = "Field `OCMS` writer - Off Chip Memory Scrambling Enable"]
pub type OCMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMINGS3_SPEC, bool, O>;
#[doc = "Field `TRR` reader - Ready to REN Low Delay"]
pub type TRR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRR` writer - Ready to REN Low Delay"]
pub type TRR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `TWB` reader - WEN High to REN to Busy"]
pub type TWB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWB` writer - WEN High to REN to Busy"]
pub type TWB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGS3_SPEC, u8, u8, 4, O>;
#[doc = "Field `RBNSEL` reader - Ready/Busy Line Selection"]
pub type RBNSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RBNSEL` writer - Ready/Busy Line Selection"]
pub type RBNSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGS3_SPEC, u8, u8, 3, O>;
#[doc = "Field `NFSEL` reader - NAND Flash Selection"]
pub type NFSEL_R = crate::BitReader<bool>;
#[doc = "Field `NFSEL` writer - NAND Flash Selection"]
pub type NFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMINGS3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - CLE to REN Low Delay"]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ALE to Data Start"]
    #[inline(always)]
    pub fn tadl(&self) -> TADL_R {
        TADL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ALE to REN Low Delay"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Off Chip Memory Scrambling Enable"]
    #[inline(always)]
    pub fn ocms(&self) -> OCMS_R {
        OCMS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Ready to REN Low Delay"]
    #[inline(always)]
    pub fn trr(&self) -> TRR_R {
        TRR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - WEN High to REN to Busy"]
    #[inline(always)]
    pub fn twb(&self) -> TWB_R {
        TWB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Ready/Busy Line Selection"]
    #[inline(always)]
    pub fn rbnsel(&self) -> RBNSEL_R {
        RBNSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - NAND Flash Selection"]
    #[inline(always)]
    pub fn nfsel(&self) -> NFSEL_R {
        NFSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLE to REN Low Delay"]
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W<0> {
        TCLR_W::new(self)
    }
    #[doc = "Bits 4:7 - ALE to Data Start"]
    #[inline(always)]
    pub fn tadl(&mut self) -> TADL_W<4> {
        TADL_W::new(self)
    }
    #[doc = "Bits 8:11 - ALE to REN Low Delay"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W<8> {
        TAR_W::new(self)
    }
    #[doc = "Bit 12 - Off Chip Memory Scrambling Enable"]
    #[inline(always)]
    pub fn ocms(&mut self) -> OCMS_W<12> {
        OCMS_W::new(self)
    }
    #[doc = "Bits 16:19 - Ready to REN Low Delay"]
    #[inline(always)]
    pub fn trr(&mut self) -> TRR_W<16> {
        TRR_W::new(self)
    }
    #[doc = "Bits 24:27 - WEN High to REN to Busy"]
    #[inline(always)]
    pub fn twb(&mut self) -> TWB_W<24> {
        TWB_W::new(self)
    }
    #[doc = "Bits 28:30 - Ready/Busy Line Selection"]
    #[inline(always)]
    pub fn rbnsel(&mut self) -> RBNSEL_W<28> {
        RBNSEL_W::new(self)
    }
    #[doc = "Bit 31 - NAND Flash Selection"]
    #[inline(always)]
    pub fn nfsel(&mut self) -> NFSEL_W<31> {
        NFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Timings Register (CS_number = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timings3](index.html) module"]
pub struct TIMINGS3_SPEC;
impl crate::RegisterSpec for TIMINGS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timings3::R](R) reader structure"]
impl crate::Readable for TIMINGS3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timings3::W](W) writer structure"]
impl crate::Writable for TIMINGS3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMINGS3 to value 0"]
impl crate::Resettable for TIMINGS3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
