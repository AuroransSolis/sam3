#[doc = "Register `TSR` reader"]
pub struct R(crate::R<TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSR` writer"]
pub struct W(crate::W<TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSR_SPEC>;
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
impl From<crate::W<TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UBR` reader - Used Bit Read"]
pub type UBR_R = crate::BitReader<bool>;
#[doc = "Field `UBR` writer - Used Bit Read"]
pub type UBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `COL` reader - Collision Occurred"]
pub type COL_R = crate::BitReader<bool>;
#[doc = "Field `COL` writer - Collision Occurred"]
pub type COL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `RLES` reader - Retry Limit exceeded"]
pub type RLES_R = crate::BitReader<bool>;
#[doc = "Field `RLES` writer - Retry Limit exceeded"]
pub type RLES_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `TGO` reader - Transmit Go"]
pub type TGO_R = crate::BitReader<bool>;
#[doc = "Field `TGO` writer - Transmit Go"]
pub type TGO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `BEX` reader - Buffers exhausted mid frame"]
pub type BEX_R = crate::BitReader<bool>;
#[doc = "Field `BEX` writer - Buffers exhausted mid frame"]
pub type BEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `COMP` reader - Transmit Complete"]
pub type COMP_R = crate::BitReader<bool>;
#[doc = "Field `COMP` writer - Transmit Complete"]
pub type COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
#[doc = "Field `UND` reader - Transmit Underrun"]
pub type UND_R = crate::BitReader<bool>;
#[doc = "Field `UND` writer - Transmit Underrun"]
pub type UND_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    pub fn ubr(&self) -> UBR_R {
        UBR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retry Limit exceeded"]
    #[inline(always)]
    pub fn rles(&self) -> RLES_R {
        RLES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    pub fn tgo(&self) -> TGO_R {
        TGO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffers exhausted mid frame"]
    #[inline(always)]
    pub fn bex(&self) -> BEX_R {
        BEX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn und(&self) -> UND_R {
        UND_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used Bit Read"]
    #[inline(always)]
    pub fn ubr(&mut self) -> UBR_W<0> {
        UBR_W::new(self)
    }
    #[doc = "Bit 1 - Collision Occurred"]
    #[inline(always)]
    pub fn col(&mut self) -> COL_W<1> {
        COL_W::new(self)
    }
    #[doc = "Bit 2 - Retry Limit exceeded"]
    #[inline(always)]
    pub fn rles(&mut self) -> RLES_W<2> {
        RLES_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Go"]
    #[inline(always)]
    pub fn tgo(&mut self) -> TGO_W<3> {
        TGO_W::new(self)
    }
    #[doc = "Bit 4 - Buffers exhausted mid frame"]
    #[inline(always)]
    pub fn bex(&mut self) -> BEX_W<4> {
        BEX_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Complete"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W<5> {
        COMP_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn und(&mut self) -> UND_W<6> {
        UND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](index.html) module"]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsr::R](R) reader structure"]
impl crate::Readable for TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsr::W](W) writer structure"]
impl crate::Writable for TSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
