#[doc = "Register `BMR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<BMR_SPEC>);
#[doc = "Register `BMR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<BMR_SPEC>);
#[doc = "Field `TC0XC0S` reader - External Clock Signal 0 Selection"]
pub type TC0XC0S_R = crate::FieldReader<u8, TC0XC0S_A>;
#[doc = "External Clock Signal 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TC0XC0S_A {
    #[doc = "0: Signal connected to XC0: TCLK0"]
    Tclk0 = 0,
    #[doc = "2: Signal connected to XC0: TIOA1"]
    Tioa1 = 2,
    #[doc = "3: Signal connected to XC0: TIOA2"]
    Tioa2 = 3,
}
impl From<TC0XC0S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC0XC0S_A) -> Self {
        variant as _
    }
}
impl TC0XC0S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TC0XC0S_A> {
        match self.bits {
            0 => Some(TC0XC0S_A::Tclk0),
            2 => Some(TC0XC0S_A::Tioa1),
            3 => Some(TC0XC0S_A::Tioa2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Tclk0`"]
    #[inline(always)]
    pub fn is_tclk0(&self) -> bool {
        *self == TC0XC0S_A::Tclk0
    }
    #[doc = "Checks if the value of the field is `Tioa1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == TC0XC0S_A::Tioa1
    }
    #[doc = "Checks if the value of the field is `Tioa2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC0XC0S_A::Tioa2
    }
}
#[doc = "Field `TC0XC0S` writer - External Clock Signal 0 Selection"]
pub type TC0XC0S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMR_SPEC, u8, TC0XC0S_A, 2, O>;
impl<'a, const O: u8> TC0XC0S_W<'a, O> {
    #[doc = "Signal connected to XC0: TCLK0"]
    #[inline(always)]
    pub fn tclk0(self) -> &'a mut W {
        self.variant(TC0XC0S_A::Tclk0)
    }
    #[doc = "Signal connected to XC0: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC0XC0S_A::Tioa1)
    }
    #[doc = "Signal connected to XC0: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC0XC0S_A::Tioa2)
    }
}
#[doc = "Field `TC1XC1S` reader - External Clock Signal 1 Selection"]
pub type TC1XC1S_R = crate::FieldReader<u8, TC1XC1S_A>;
#[doc = "External Clock Signal 1 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TC1XC1S_A {
    #[doc = "0: Signal connected to XC1: TCLK1"]
    Tclk1 = 0,
    #[doc = "2: Signal connected to XC1: TIOA0"]
    Tioa0 = 2,
    #[doc = "3: Signal connected to XC1: TIOA2"]
    Tioa2 = 3,
}
impl From<TC1XC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC1XC1S_A) -> Self {
        variant as _
    }
}
impl TC1XC1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TC1XC1S_A> {
        match self.bits {
            0 => Some(TC1XC1S_A::Tclk1),
            2 => Some(TC1XC1S_A::Tioa0),
            3 => Some(TC1XC1S_A::Tioa2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Tclk1`"]
    #[inline(always)]
    pub fn is_tclk1(&self) -> bool {
        *self == TC1XC1S_A::Tclk1
    }
    #[doc = "Checks if the value of the field is `Tioa0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == TC1XC1S_A::Tioa0
    }
    #[doc = "Checks if the value of the field is `Tioa2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC1XC1S_A::Tioa2
    }
}
#[doc = "Field `TC1XC1S` writer - External Clock Signal 1 Selection"]
pub type TC1XC1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMR_SPEC, u8, TC1XC1S_A, 2, O>;
impl<'a, const O: u8> TC1XC1S_W<'a, O> {
    #[doc = "Signal connected to XC1: TCLK1"]
    #[inline(always)]
    pub fn tclk1(self) -> &'a mut W {
        self.variant(TC1XC1S_A::Tclk1)
    }
    #[doc = "Signal connected to XC1: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC1XC1S_A::Tioa0)
    }
    #[doc = "Signal connected to XC1: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC1XC1S_A::Tioa2)
    }
}
#[doc = "Field `TC2XC2S` reader - External Clock Signal 2 Selection"]
pub type TC2XC2S_R = crate::FieldReader<u8, TC2XC2S_A>;
#[doc = "External Clock Signal 2 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TC2XC2S_A {
    #[doc = "0: Signal connected to XC2: TCLK2"]
    Tclk2 = 0,
    #[doc = "2: Signal connected to XC2: TIOA1"]
    Tioa1 = 2,
    #[doc = "3: Signal connected to XC2: TIOA2"]
    Tioa2 = 3,
}
impl From<TC2XC2S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC2XC2S_A) -> Self {
        variant as _
    }
}
impl TC2XC2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TC2XC2S_A> {
        match self.bits {
            0 => Some(TC2XC2S_A::Tclk2),
            2 => Some(TC2XC2S_A::Tioa1),
            3 => Some(TC2XC2S_A::Tioa2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Tclk2`"]
    #[inline(always)]
    pub fn is_tclk2(&self) -> bool {
        *self == TC2XC2S_A::Tclk2
    }
    #[doc = "Checks if the value of the field is `Tioa1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == TC2XC2S_A::Tioa1
    }
    #[doc = "Checks if the value of the field is `Tioa2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC2XC2S_A::Tioa2
    }
}
#[doc = "Field `TC2XC2S` writer - External Clock Signal 2 Selection"]
pub type TC2XC2S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMR_SPEC, u8, TC2XC2S_A, 2, O>;
impl<'a, const O: u8> TC2XC2S_W<'a, O> {
    #[doc = "Signal connected to XC2: TCLK2"]
    #[inline(always)]
    pub fn tclk2(self) -> &'a mut W {
        self.variant(TC2XC2S_A::Tclk2)
    }
    #[doc = "Signal connected to XC2: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC2XC2S_A::Tioa1)
    }
    #[doc = "Signal connected to XC2: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC2XC2S_A::Tioa2)
    }
}
#[doc = "Field `QDEN` reader - Quadrature Decoder ENabled"]
pub type QDEN_R = crate::BitReader<bool>;
#[doc = "Field `QDEN` writer - Quadrature Decoder ENabled"]
pub type QDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `POSEN` reader - POSition ENabled"]
pub type POSEN_R = crate::BitReader<bool>;
#[doc = "Field `POSEN` writer - POSition ENabled"]
pub type POSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `SPEEDEN` reader - SPEED ENabled"]
pub type SPEEDEN_R = crate::BitReader<bool>;
#[doc = "Field `SPEEDEN` writer - SPEED ENabled"]
pub type SPEEDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `QDTRANS` reader - Quadrature Decoding TRANSparent"]
pub type QDTRANS_R = crate::BitReader<bool>;
#[doc = "Field `QDTRANS` writer - Quadrature Decoding TRANSparent"]
pub type QDTRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `EDGPHA` reader - EDGe on PHA count mode"]
pub type EDGPHA_R = crate::BitReader<bool>;
#[doc = "Field `EDGPHA` writer - EDGe on PHA count mode"]
pub type EDGPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `INVA` reader - INVerted phA"]
pub type INVA_R = crate::BitReader<bool>;
#[doc = "Field `INVA` writer - INVerted phA"]
pub type INVA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `INVB` reader - INVerted phB"]
pub type INVB_R = crate::BitReader<bool>;
#[doc = "Field `INVB` writer - INVerted phB"]
pub type INVB_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `INVIDX` reader - INVerted InDeX"]
pub type INVIDX_R = crate::BitReader<bool>;
#[doc = "Field `INVIDX` writer - INVerted InDeX"]
pub type INVIDX_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `SWAP` reader - SWAP PHA and PHB"]
pub type SWAP_R = crate::BitReader<bool>;
#[doc = "Field `SWAP` writer - SWAP PHA and PHB"]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `IDXPHB` reader - InDeX pin is PHB pin"]
pub type IDXPHB_R = crate::BitReader<bool>;
#[doc = "Field `IDXPHB` writer - InDeX pin is PHB pin"]
pub type IDXPHB_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `FILTER` reader - "]
pub type FILTER_R = crate::BitReader<bool>;
#[doc = "Field `FILTER` writer - "]
pub type FILTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMR_SPEC, bool, O>;
#[doc = "Field `MAXFILT` reader - MAXimum FILTer"]
pub type MAXFILT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAXFILT` writer - MAXimum FILTer"]
pub type MAXFILT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&self) -> TC0XC0S_R {
        TC0XC0S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&self) -> TC1XC1S_R {
        TC1XC1S_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&self) -> TC2XC2S_R {
        TC2XC2S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Quadrature Decoder ENabled"]
    #[inline(always)]
    pub fn qden(&self) -> QDEN_R {
        QDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - POSition ENabled"]
    #[inline(always)]
    pub fn posen(&self) -> POSEN_R {
        POSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPEED ENabled"]
    #[inline(always)]
    pub fn speeden(&self) -> SPEEDEN_R {
        SPEEDEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Quadrature Decoding TRANSparent"]
    #[inline(always)]
    pub fn qdtrans(&self) -> QDTRANS_R {
        QDTRANS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EDGe on PHA count mode"]
    #[inline(always)]
    pub fn edgpha(&self) -> EDGPHA_R {
        EDGPHA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - INVerted phA"]
    #[inline(always)]
    pub fn inva(&self) -> INVA_R {
        INVA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - INVerted phB"]
    #[inline(always)]
    pub fn invb(&self) -> INVB_R {
        INVB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - INVerted InDeX"]
    #[inline(always)]
    pub fn invidx(&self) -> INVIDX_R {
        INVIDX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SWAP PHA and PHB"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - InDeX pin is PHB pin"]
    #[inline(always)]
    pub fn idxphb(&self) -> IDXPHB_R {
        IDXPHB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:25 - MAXimum FILTer"]
    #[inline(always)]
    pub fn maxfilt(&self) -> MAXFILT_R {
        MAXFILT_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc0xc0s(&mut self) -> TC0XC0S_W<0> {
        TC0XC0S_W::new(self)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc1xc1s(&mut self) -> TC1XC1S_W<2> {
        TC1XC1S_W::new(self)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tc2xc2s(&mut self) -> TC2XC2S_W<4> {
        TC2XC2S_W::new(self)
    }
    #[doc = "Bit 8 - Quadrature Decoder ENabled"]
    #[inline(always)]
    #[must_use]
    pub fn qden(&mut self) -> QDEN_W<8> {
        QDEN_W::new(self)
    }
    #[doc = "Bit 9 - POSition ENabled"]
    #[inline(always)]
    #[must_use]
    pub fn posen(&mut self) -> POSEN_W<9> {
        POSEN_W::new(self)
    }
    #[doc = "Bit 10 - SPEED ENabled"]
    #[inline(always)]
    #[must_use]
    pub fn speeden(&mut self) -> SPEEDEN_W<10> {
        SPEEDEN_W::new(self)
    }
    #[doc = "Bit 11 - Quadrature Decoding TRANSparent"]
    #[inline(always)]
    #[must_use]
    pub fn qdtrans(&mut self) -> QDTRANS_W<11> {
        QDTRANS_W::new(self)
    }
    #[doc = "Bit 12 - EDGe on PHA count mode"]
    #[inline(always)]
    #[must_use]
    pub fn edgpha(&mut self) -> EDGPHA_W<12> {
        EDGPHA_W::new(self)
    }
    #[doc = "Bit 13 - INVerted phA"]
    #[inline(always)]
    #[must_use]
    pub fn inva(&mut self) -> INVA_W<13> {
        INVA_W::new(self)
    }
    #[doc = "Bit 14 - INVerted phB"]
    #[inline(always)]
    #[must_use]
    pub fn invb(&mut self) -> INVB_W<14> {
        INVB_W::new(self)
    }
    #[doc = "Bit 15 - INVerted InDeX"]
    #[inline(always)]
    #[must_use]
    pub fn invidx(&mut self) -> INVIDX_W<15> {
        INVIDX_W::new(self)
    }
    #[doc = "Bit 16 - SWAP PHA and PHB"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<16> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 17 - InDeX pin is PHB pin"]
    #[inline(always)]
    #[must_use]
    pub fn idxphb(&mut self) -> IDXPHB_W<17> {
        IDXPHB_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<19> {
        FILTER_W::new(self)
    }
    #[doc = "Bits 20:25 - MAXimum FILTer"]
    #[inline(always)]
    #[must_use]
    pub fn maxfilt(&mut self) -> MAXFILT_W<20> {
        MAXFILT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmr](index.html) module"]
pub struct BMR_SPEC;
impl crate::RegisterSpec for BMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmr::R](R) reader structure"]
impl crate::Readable for BMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmr::W](W) writer structure"]
impl crate::Writable for BMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMR to value 0"]
impl crate::Resettable for BMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
