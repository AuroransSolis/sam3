#[doc = "Register `COR` reader"]
pub struct R(crate::R<COR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COR` writer"]
pub struct W(crate::W<COR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COR_SPEC>;
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
impl From<crate::W<COR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFF0` reader - Offset for channel 0"]
pub type OFF0_R = crate::BitReader<bool>;
#[doc = "Field `OFF0` writer - Offset for channel 0"]
pub type OFF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF1` reader - Offset for channel 1"]
pub type OFF1_R = crate::BitReader<bool>;
#[doc = "Field `OFF1` writer - Offset for channel 1"]
pub type OFF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF2` reader - Offset for channel 2"]
pub type OFF2_R = crate::BitReader<bool>;
#[doc = "Field `OFF2` writer - Offset for channel 2"]
pub type OFF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF3` reader - Offset for channel 3"]
pub type OFF3_R = crate::BitReader<bool>;
#[doc = "Field `OFF3` writer - Offset for channel 3"]
pub type OFF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF4` reader - Offset for channel 4"]
pub type OFF4_R = crate::BitReader<bool>;
#[doc = "Field `OFF4` writer - Offset for channel 4"]
pub type OFF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF5` reader - Offset for channel 5"]
pub type OFF5_R = crate::BitReader<bool>;
#[doc = "Field `OFF5` writer - Offset for channel 5"]
pub type OFF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF6` reader - Offset for channel 6"]
pub type OFF6_R = crate::BitReader<bool>;
#[doc = "Field `OFF6` writer - Offset for channel 6"]
pub type OFF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF7` reader - Offset for channel 7"]
pub type OFF7_R = crate::BitReader<bool>;
#[doc = "Field `OFF7` writer - Offset for channel 7"]
pub type OFF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF8` reader - Offset for channel 8"]
pub type OFF8_R = crate::BitReader<bool>;
#[doc = "Field `OFF8` writer - Offset for channel 8"]
pub type OFF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF9` reader - Offset for channel 9"]
pub type OFF9_R = crate::BitReader<bool>;
#[doc = "Field `OFF9` writer - Offset for channel 9"]
pub type OFF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF10` reader - Offset for channel 10"]
pub type OFF10_R = crate::BitReader<bool>;
#[doc = "Field `OFF10` writer - Offset for channel 10"]
pub type OFF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF11` reader - Offset for channel 11"]
pub type OFF11_R = crate::BitReader<bool>;
#[doc = "Field `OFF11` writer - Offset for channel 11"]
pub type OFF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF12` reader - Offset for channel 12"]
pub type OFF12_R = crate::BitReader<bool>;
#[doc = "Field `OFF12` writer - Offset for channel 12"]
pub type OFF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF13` reader - Offset for channel 13"]
pub type OFF13_R = crate::BitReader<bool>;
#[doc = "Field `OFF13` writer - Offset for channel 13"]
pub type OFF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF14` reader - Offset for channel 14"]
pub type OFF14_R = crate::BitReader<bool>;
#[doc = "Field `OFF14` writer - Offset for channel 14"]
pub type OFF14_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `OFF15` reader - Offset for channel 15"]
pub type OFF15_R = crate::BitReader<bool>;
#[doc = "Field `OFF15` writer - Offset for channel 15"]
pub type OFF15_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF0` reader - Differential inputs for channel 0"]
pub type DIFF0_R = crate::BitReader<bool>;
#[doc = "Field `DIFF0` writer - Differential inputs for channel 0"]
pub type DIFF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF1` reader - Differential inputs for channel 1"]
pub type DIFF1_R = crate::BitReader<bool>;
#[doc = "Field `DIFF1` writer - Differential inputs for channel 1"]
pub type DIFF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF2` reader - Differential inputs for channel 2"]
pub type DIFF2_R = crate::BitReader<bool>;
#[doc = "Field `DIFF2` writer - Differential inputs for channel 2"]
pub type DIFF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF3` reader - Differential inputs for channel 3"]
pub type DIFF3_R = crate::BitReader<bool>;
#[doc = "Field `DIFF3` writer - Differential inputs for channel 3"]
pub type DIFF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF4` reader - Differential inputs for channel 4"]
pub type DIFF4_R = crate::BitReader<bool>;
#[doc = "Field `DIFF4` writer - Differential inputs for channel 4"]
pub type DIFF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF5` reader - Differential inputs for channel 5"]
pub type DIFF5_R = crate::BitReader<bool>;
#[doc = "Field `DIFF5` writer - Differential inputs for channel 5"]
pub type DIFF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF6` reader - Differential inputs for channel 6"]
pub type DIFF6_R = crate::BitReader<bool>;
#[doc = "Field `DIFF6` writer - Differential inputs for channel 6"]
pub type DIFF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF7` reader - Differential inputs for channel 7"]
pub type DIFF7_R = crate::BitReader<bool>;
#[doc = "Field `DIFF7` writer - Differential inputs for channel 7"]
pub type DIFF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF8` reader - Differential inputs for channel 8"]
pub type DIFF8_R = crate::BitReader<bool>;
#[doc = "Field `DIFF8` writer - Differential inputs for channel 8"]
pub type DIFF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF9` reader - Differential inputs for channel 9"]
pub type DIFF9_R = crate::BitReader<bool>;
#[doc = "Field `DIFF9` writer - Differential inputs for channel 9"]
pub type DIFF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF10` reader - Differential inputs for channel 10"]
pub type DIFF10_R = crate::BitReader<bool>;
#[doc = "Field `DIFF10` writer - Differential inputs for channel 10"]
pub type DIFF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF11` reader - Differential inputs for channel 11"]
pub type DIFF11_R = crate::BitReader<bool>;
#[doc = "Field `DIFF11` writer - Differential inputs for channel 11"]
pub type DIFF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF12` reader - Differential inputs for channel 12"]
pub type DIFF12_R = crate::BitReader<bool>;
#[doc = "Field `DIFF12` writer - Differential inputs for channel 12"]
pub type DIFF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF13` reader - Differential inputs for channel 13"]
pub type DIFF13_R = crate::BitReader<bool>;
#[doc = "Field `DIFF13` writer - Differential inputs for channel 13"]
pub type DIFF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF14` reader - Differential inputs for channel 14"]
pub type DIFF14_R = crate::BitReader<bool>;
#[doc = "Field `DIFF14` writer - Differential inputs for channel 14"]
pub type DIFF14_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
#[doc = "Field `DIFF15` reader - Differential inputs for channel 15"]
pub type DIFF15_R = crate::BitReader<bool>;
#[doc = "Field `DIFF15` writer - Differential inputs for channel 15"]
pub type DIFF15_W<'a, const O: u8> = crate::BitWriter<'a, u32, COR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Offset for channel 0"]
    #[inline(always)]
    pub fn off0(&self) -> OFF0_R {
        OFF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Offset for channel 1"]
    #[inline(always)]
    pub fn off1(&self) -> OFF1_R {
        OFF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Offset for channel 2"]
    #[inline(always)]
    pub fn off2(&self) -> OFF2_R {
        OFF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Offset for channel 3"]
    #[inline(always)]
    pub fn off3(&self) -> OFF3_R {
        OFF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Offset for channel 4"]
    #[inline(always)]
    pub fn off4(&self) -> OFF4_R {
        OFF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Offset for channel 5"]
    #[inline(always)]
    pub fn off5(&self) -> OFF5_R {
        OFF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Offset for channel 6"]
    #[inline(always)]
    pub fn off6(&self) -> OFF6_R {
        OFF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Offset for channel 7"]
    #[inline(always)]
    pub fn off7(&self) -> OFF7_R {
        OFF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Offset for channel 8"]
    #[inline(always)]
    pub fn off8(&self) -> OFF8_R {
        OFF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Offset for channel 9"]
    #[inline(always)]
    pub fn off9(&self) -> OFF9_R {
        OFF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Offset for channel 10"]
    #[inline(always)]
    pub fn off10(&self) -> OFF10_R {
        OFF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Offset for channel 11"]
    #[inline(always)]
    pub fn off11(&self) -> OFF11_R {
        OFF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Offset for channel 12"]
    #[inline(always)]
    pub fn off12(&self) -> OFF12_R {
        OFF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Offset for channel 13"]
    #[inline(always)]
    pub fn off13(&self) -> OFF13_R {
        OFF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Offset for channel 14"]
    #[inline(always)]
    pub fn off14(&self) -> OFF14_R {
        OFF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Offset for channel 15"]
    #[inline(always)]
    pub fn off15(&self) -> OFF15_R {
        OFF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Differential inputs for channel 0"]
    #[inline(always)]
    pub fn diff0(&self) -> DIFF0_R {
        DIFF0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Differential inputs for channel 1"]
    #[inline(always)]
    pub fn diff1(&self) -> DIFF1_R {
        DIFF1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Differential inputs for channel 2"]
    #[inline(always)]
    pub fn diff2(&self) -> DIFF2_R {
        DIFF2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Differential inputs for channel 3"]
    #[inline(always)]
    pub fn diff3(&self) -> DIFF3_R {
        DIFF3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Differential inputs for channel 4"]
    #[inline(always)]
    pub fn diff4(&self) -> DIFF4_R {
        DIFF4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Differential inputs for channel 5"]
    #[inline(always)]
    pub fn diff5(&self) -> DIFF5_R {
        DIFF5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Differential inputs for channel 6"]
    #[inline(always)]
    pub fn diff6(&self) -> DIFF6_R {
        DIFF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Differential inputs for channel 7"]
    #[inline(always)]
    pub fn diff7(&self) -> DIFF7_R {
        DIFF7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Differential inputs for channel 8"]
    #[inline(always)]
    pub fn diff8(&self) -> DIFF8_R {
        DIFF8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Differential inputs for channel 9"]
    #[inline(always)]
    pub fn diff9(&self) -> DIFF9_R {
        DIFF9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Differential inputs for channel 10"]
    #[inline(always)]
    pub fn diff10(&self) -> DIFF10_R {
        DIFF10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Differential inputs for channel 11"]
    #[inline(always)]
    pub fn diff11(&self) -> DIFF11_R {
        DIFF11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Differential inputs for channel 12"]
    #[inline(always)]
    pub fn diff12(&self) -> DIFF12_R {
        DIFF12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Differential inputs for channel 13"]
    #[inline(always)]
    pub fn diff13(&self) -> DIFF13_R {
        DIFF13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Differential inputs for channel 14"]
    #[inline(always)]
    pub fn diff14(&self) -> DIFF14_R {
        DIFF14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Differential inputs for channel 15"]
    #[inline(always)]
    pub fn diff15(&self) -> DIFF15_R {
        DIFF15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Offset for channel 0"]
    #[inline(always)]
    pub fn off0(&mut self) -> OFF0_W<0> {
        OFF0_W::new(self)
    }
    #[doc = "Bit 1 - Offset for channel 1"]
    #[inline(always)]
    pub fn off1(&mut self) -> OFF1_W<1> {
        OFF1_W::new(self)
    }
    #[doc = "Bit 2 - Offset for channel 2"]
    #[inline(always)]
    pub fn off2(&mut self) -> OFF2_W<2> {
        OFF2_W::new(self)
    }
    #[doc = "Bit 3 - Offset for channel 3"]
    #[inline(always)]
    pub fn off3(&mut self) -> OFF3_W<3> {
        OFF3_W::new(self)
    }
    #[doc = "Bit 4 - Offset for channel 4"]
    #[inline(always)]
    pub fn off4(&mut self) -> OFF4_W<4> {
        OFF4_W::new(self)
    }
    #[doc = "Bit 5 - Offset for channel 5"]
    #[inline(always)]
    pub fn off5(&mut self) -> OFF5_W<5> {
        OFF5_W::new(self)
    }
    #[doc = "Bit 6 - Offset for channel 6"]
    #[inline(always)]
    pub fn off6(&mut self) -> OFF6_W<6> {
        OFF6_W::new(self)
    }
    #[doc = "Bit 7 - Offset for channel 7"]
    #[inline(always)]
    pub fn off7(&mut self) -> OFF7_W<7> {
        OFF7_W::new(self)
    }
    #[doc = "Bit 8 - Offset for channel 8"]
    #[inline(always)]
    pub fn off8(&mut self) -> OFF8_W<8> {
        OFF8_W::new(self)
    }
    #[doc = "Bit 9 - Offset for channel 9"]
    #[inline(always)]
    pub fn off9(&mut self) -> OFF9_W<9> {
        OFF9_W::new(self)
    }
    #[doc = "Bit 10 - Offset for channel 10"]
    #[inline(always)]
    pub fn off10(&mut self) -> OFF10_W<10> {
        OFF10_W::new(self)
    }
    #[doc = "Bit 11 - Offset for channel 11"]
    #[inline(always)]
    pub fn off11(&mut self) -> OFF11_W<11> {
        OFF11_W::new(self)
    }
    #[doc = "Bit 12 - Offset for channel 12"]
    #[inline(always)]
    pub fn off12(&mut self) -> OFF12_W<12> {
        OFF12_W::new(self)
    }
    #[doc = "Bit 13 - Offset for channel 13"]
    #[inline(always)]
    pub fn off13(&mut self) -> OFF13_W<13> {
        OFF13_W::new(self)
    }
    #[doc = "Bit 14 - Offset for channel 14"]
    #[inline(always)]
    pub fn off14(&mut self) -> OFF14_W<14> {
        OFF14_W::new(self)
    }
    #[doc = "Bit 15 - Offset for channel 15"]
    #[inline(always)]
    pub fn off15(&mut self) -> OFF15_W<15> {
        OFF15_W::new(self)
    }
    #[doc = "Bit 16 - Differential inputs for channel 0"]
    #[inline(always)]
    pub fn diff0(&mut self) -> DIFF0_W<16> {
        DIFF0_W::new(self)
    }
    #[doc = "Bit 17 - Differential inputs for channel 1"]
    #[inline(always)]
    pub fn diff1(&mut self) -> DIFF1_W<17> {
        DIFF1_W::new(self)
    }
    #[doc = "Bit 18 - Differential inputs for channel 2"]
    #[inline(always)]
    pub fn diff2(&mut self) -> DIFF2_W<18> {
        DIFF2_W::new(self)
    }
    #[doc = "Bit 19 - Differential inputs for channel 3"]
    #[inline(always)]
    pub fn diff3(&mut self) -> DIFF3_W<19> {
        DIFF3_W::new(self)
    }
    #[doc = "Bit 20 - Differential inputs for channel 4"]
    #[inline(always)]
    pub fn diff4(&mut self) -> DIFF4_W<20> {
        DIFF4_W::new(self)
    }
    #[doc = "Bit 21 - Differential inputs for channel 5"]
    #[inline(always)]
    pub fn diff5(&mut self) -> DIFF5_W<21> {
        DIFF5_W::new(self)
    }
    #[doc = "Bit 22 - Differential inputs for channel 6"]
    #[inline(always)]
    pub fn diff6(&mut self) -> DIFF6_W<22> {
        DIFF6_W::new(self)
    }
    #[doc = "Bit 23 - Differential inputs for channel 7"]
    #[inline(always)]
    pub fn diff7(&mut self) -> DIFF7_W<23> {
        DIFF7_W::new(self)
    }
    #[doc = "Bit 24 - Differential inputs for channel 8"]
    #[inline(always)]
    pub fn diff8(&mut self) -> DIFF8_W<24> {
        DIFF8_W::new(self)
    }
    #[doc = "Bit 25 - Differential inputs for channel 9"]
    #[inline(always)]
    pub fn diff9(&mut self) -> DIFF9_W<25> {
        DIFF9_W::new(self)
    }
    #[doc = "Bit 26 - Differential inputs for channel 10"]
    #[inline(always)]
    pub fn diff10(&mut self) -> DIFF10_W<26> {
        DIFF10_W::new(self)
    }
    #[doc = "Bit 27 - Differential inputs for channel 11"]
    #[inline(always)]
    pub fn diff11(&mut self) -> DIFF11_W<27> {
        DIFF11_W::new(self)
    }
    #[doc = "Bit 28 - Differential inputs for channel 12"]
    #[inline(always)]
    pub fn diff12(&mut self) -> DIFF12_W<28> {
        DIFF12_W::new(self)
    }
    #[doc = "Bit 29 - Differential inputs for channel 13"]
    #[inline(always)]
    pub fn diff13(&mut self) -> DIFF13_W<29> {
        DIFF13_W::new(self)
    }
    #[doc = "Bit 30 - Differential inputs for channel 14"]
    #[inline(always)]
    pub fn diff14(&mut self) -> DIFF14_W<30> {
        DIFF14_W::new(self)
    }
    #[doc = "Bit 31 - Differential inputs for channel 15"]
    #[inline(always)]
    pub fn diff15(&mut self) -> DIFF15_W<31> {
        DIFF15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cor](index.html) module"]
pub struct COR_SPEC;
impl crate::RegisterSpec for COR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cor::R](R) reader structure"]
impl crate::Readable for COR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cor::W](W) writer structure"]
impl crate::Writable for COR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COR to value 0"]
impl crate::Resettable for COR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
