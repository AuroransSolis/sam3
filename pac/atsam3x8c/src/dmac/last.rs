#[doc = "Register `LAST` reader"]
pub struct R(crate::R<LAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAST` writer"]
pub struct W(crate::W<LAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAST_SPEC>;
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
impl From<crate::W<LAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAST0` reader - Source Last"]
pub type SLAST0_R = crate::BitReader<bool>;
#[doc = "Field `SLAST0` writer - Source Last"]
pub type SLAST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `DLAST0` reader - Destination Last"]
pub type DLAST0_R = crate::BitReader<bool>;
#[doc = "Field `DLAST0` writer - Destination Last"]
pub type DLAST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `SLAST1` reader - Source Last"]
pub type SLAST1_R = crate::BitReader<bool>;
#[doc = "Field `SLAST1` writer - Source Last"]
pub type SLAST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `DLAST1` reader - Destination Last"]
pub type DLAST1_R = crate::BitReader<bool>;
#[doc = "Field `DLAST1` writer - Destination Last"]
pub type DLAST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `SLAST2` reader - Source Last"]
pub type SLAST2_R = crate::BitReader<bool>;
#[doc = "Field `SLAST2` writer - Source Last"]
pub type SLAST2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `DLAST2` reader - Destination Last"]
pub type DLAST2_R = crate::BitReader<bool>;
#[doc = "Field `DLAST2` writer - Destination Last"]
pub type DLAST2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `SLAST3` reader - Source Last"]
pub type SLAST3_R = crate::BitReader<bool>;
#[doc = "Field `SLAST3` writer - Source Last"]
pub type SLAST3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `DLAST3` reader - Destination Last"]
pub type DLAST3_R = crate::BitReader<bool>;
#[doc = "Field `DLAST3` writer - Destination Last"]
pub type DLAST3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `SLAST4` reader - Source Last"]
pub type SLAST4_R = crate::BitReader<bool>;
#[doc = "Field `SLAST4` writer - Source Last"]
pub type SLAST4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `DLAST4` reader - Destination Last"]
pub type DLAST4_R = crate::BitReader<bool>;
#[doc = "Field `DLAST4` writer - Destination Last"]
pub type DLAST4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `SLAST5` reader - Source Last"]
pub type SLAST5_R = crate::BitReader<bool>;
#[doc = "Field `SLAST5` writer - Source Last"]
pub type SLAST5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
#[doc = "Field `DLAST5` reader - Destination Last"]
pub type DLAST5_R = crate::BitReader<bool>;
#[doc = "Field `DLAST5` writer - Destination Last"]
pub type DLAST5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Source Last"]
    #[inline(always)]
    pub fn slast0(&self) -> SLAST0_R {
        SLAST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Destination Last"]
    #[inline(always)]
    pub fn dlast0(&self) -> DLAST0_R {
        DLAST0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source Last"]
    #[inline(always)]
    pub fn slast1(&self) -> SLAST1_R {
        SLAST1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Last"]
    #[inline(always)]
    pub fn dlast1(&self) -> DLAST1_R {
        DLAST1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source Last"]
    #[inline(always)]
    pub fn slast2(&self) -> SLAST2_R {
        SLAST2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Last"]
    #[inline(always)]
    pub fn dlast2(&self) -> DLAST2_R {
        DLAST2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Last"]
    #[inline(always)]
    pub fn slast3(&self) -> SLAST3_R {
        SLAST3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Destination Last"]
    #[inline(always)]
    pub fn dlast3(&self) -> DLAST3_R {
        DLAST3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source Last"]
    #[inline(always)]
    pub fn slast4(&self) -> SLAST4_R {
        SLAST4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Destination Last"]
    #[inline(always)]
    pub fn dlast4(&self) -> DLAST4_R {
        DLAST4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Source Last"]
    #[inline(always)]
    pub fn slast5(&self) -> SLAST5_R {
        SLAST5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Destination Last"]
    #[inline(always)]
    pub fn dlast5(&self) -> DLAST5_R {
        DLAST5_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Last"]
    #[inline(always)]
    pub fn slast0(&mut self) -> SLAST0_W<0> {
        SLAST0_W::new(self)
    }
    #[doc = "Bit 1 - Destination Last"]
    #[inline(always)]
    pub fn dlast0(&mut self) -> DLAST0_W<1> {
        DLAST0_W::new(self)
    }
    #[doc = "Bit 2 - Source Last"]
    #[inline(always)]
    pub fn slast1(&mut self) -> SLAST1_W<2> {
        SLAST1_W::new(self)
    }
    #[doc = "Bit 3 - Destination Last"]
    #[inline(always)]
    pub fn dlast1(&mut self) -> DLAST1_W<3> {
        DLAST1_W::new(self)
    }
    #[doc = "Bit 4 - Source Last"]
    #[inline(always)]
    pub fn slast2(&mut self) -> SLAST2_W<4> {
        SLAST2_W::new(self)
    }
    #[doc = "Bit 5 - Destination Last"]
    #[inline(always)]
    pub fn dlast2(&mut self) -> DLAST2_W<5> {
        DLAST2_W::new(self)
    }
    #[doc = "Bit 6 - Source Last"]
    #[inline(always)]
    pub fn slast3(&mut self) -> SLAST3_W<6> {
        SLAST3_W::new(self)
    }
    #[doc = "Bit 7 - Destination Last"]
    #[inline(always)]
    pub fn dlast3(&mut self) -> DLAST3_W<7> {
        DLAST3_W::new(self)
    }
    #[doc = "Bit 8 - Source Last"]
    #[inline(always)]
    pub fn slast4(&mut self) -> SLAST4_W<8> {
        SLAST4_W::new(self)
    }
    #[doc = "Bit 9 - Destination Last"]
    #[inline(always)]
    pub fn dlast4(&mut self) -> DLAST4_W<9> {
        DLAST4_W::new(self)
    }
    #[doc = "Bit 10 - Source Last"]
    #[inline(always)]
    pub fn slast5(&mut self) -> SLAST5_W<10> {
        SLAST5_W::new(self)
    }
    #[doc = "Bit 11 - Destination Last"]
    #[inline(always)]
    pub fn dlast5(&mut self) -> DLAST5_W<11> {
        DLAST5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Software Last Transfer Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [last](index.html) module"]
pub struct LAST_SPEC;
impl crate::RegisterSpec for LAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [last::R](R) reader structure"]
impl crate::Readable for LAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [last::W](W) writer structure"]
impl crate::Writable for LAST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LAST to value 0"]
impl crate::Resettable for LAST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
