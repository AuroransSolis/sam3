#[doc = "Register `SMMR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<SMMR_SPEC>);
#[doc = "Register `SMMR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<SMMR_SPEC>);
#[doc = "Field `SMTH` reader - Supply Monitor Threshold"]
pub type SMTH_R = crate::FieldReader<u8, SMTH_A>;
#[doc = "Supply Monitor Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMTH_A {
    #[doc = "0: 1.9 V"]
    _19v = 0,
    #[doc = "1: 2.0 V"]
    _20v = 1,
    #[doc = "2: 2.1 V"]
    _21v = 2,
    #[doc = "3: 2.2 V"]
    _22v = 3,
    #[doc = "4: 2.3 V"]
    _23v = 4,
    #[doc = "5: 2.4 V"]
    _24v = 5,
    #[doc = "6: 2.5 V"]
    _25v = 6,
    #[doc = "7: 2.6 V"]
    _26v = 7,
    #[doc = "8: 2.7 V"]
    _27v = 8,
    #[doc = "9: 2.8 V"]
    _28v = 9,
    #[doc = "10: 2.9 V"]
    _29v = 10,
    #[doc = "11: 3.0 V"]
    _30v = 11,
    #[doc = "12: 3.1 V"]
    _31v = 12,
    #[doc = "13: 3.2 V"]
    _32v = 13,
    #[doc = "14: 3.3 V"]
    _33v = 14,
    #[doc = "15: 3.4 V"]
    _34v = 15,
}
impl From<SMTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SMTH_A) -> Self {
        variant as _
    }
}
impl SMTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMTH_A {
        match self.bits {
            0 => SMTH_A::_19v,
            1 => SMTH_A::_20v,
            2 => SMTH_A::_21v,
            3 => SMTH_A::_22v,
            4 => SMTH_A::_23v,
            5 => SMTH_A::_24v,
            6 => SMTH_A::_25v,
            7 => SMTH_A::_26v,
            8 => SMTH_A::_27v,
            9 => SMTH_A::_28v,
            10 => SMTH_A::_29v,
            11 => SMTH_A::_30v,
            12 => SMTH_A::_31v,
            13 => SMTH_A::_32v,
            14 => SMTH_A::_33v,
            15 => SMTH_A::_34v,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_19v`"]
    #[inline(always)]
    pub fn is_1_9v(&self) -> bool {
        *self == SMTH_A::_19v
    }
    #[doc = "Checks if the value of the field is `_20v`"]
    #[inline(always)]
    pub fn is_2_0v(&self) -> bool {
        *self == SMTH_A::_20v
    }
    #[doc = "Checks if the value of the field is `_21v`"]
    #[inline(always)]
    pub fn is_2_1v(&self) -> bool {
        *self == SMTH_A::_21v
    }
    #[doc = "Checks if the value of the field is `_22v`"]
    #[inline(always)]
    pub fn is_2_2v(&self) -> bool {
        *self == SMTH_A::_22v
    }
    #[doc = "Checks if the value of the field is `_23v`"]
    #[inline(always)]
    pub fn is_2_3v(&self) -> bool {
        *self == SMTH_A::_23v
    }
    #[doc = "Checks if the value of the field is `_24v`"]
    #[inline(always)]
    pub fn is_2_4v(&self) -> bool {
        *self == SMTH_A::_24v
    }
    #[doc = "Checks if the value of the field is `_25v`"]
    #[inline(always)]
    pub fn is_2_5v(&self) -> bool {
        *self == SMTH_A::_25v
    }
    #[doc = "Checks if the value of the field is `_26v`"]
    #[inline(always)]
    pub fn is_2_6v(&self) -> bool {
        *self == SMTH_A::_26v
    }
    #[doc = "Checks if the value of the field is `_27v`"]
    #[inline(always)]
    pub fn is_2_7v(&self) -> bool {
        *self == SMTH_A::_27v
    }
    #[doc = "Checks if the value of the field is `_28v`"]
    #[inline(always)]
    pub fn is_2_8v(&self) -> bool {
        *self == SMTH_A::_28v
    }
    #[doc = "Checks if the value of the field is `_29v`"]
    #[inline(always)]
    pub fn is_2_9v(&self) -> bool {
        *self == SMTH_A::_29v
    }
    #[doc = "Checks if the value of the field is `_30v`"]
    #[inline(always)]
    pub fn is_3_0v(&self) -> bool {
        *self == SMTH_A::_30v
    }
    #[doc = "Checks if the value of the field is `_31v`"]
    #[inline(always)]
    pub fn is_3_1v(&self) -> bool {
        *self == SMTH_A::_31v
    }
    #[doc = "Checks if the value of the field is `_32v`"]
    #[inline(always)]
    pub fn is_3_2v(&self) -> bool {
        *self == SMTH_A::_32v
    }
    #[doc = "Checks if the value of the field is `_33v`"]
    #[inline(always)]
    pub fn is_3_3v(&self) -> bool {
        *self == SMTH_A::_33v
    }
    #[doc = "Checks if the value of the field is `_34v`"]
    #[inline(always)]
    pub fn is_3_4v(&self) -> bool {
        *self == SMTH_A::_34v
    }
}
#[doc = "Field `SMTH` writer - Supply Monitor Threshold"]
pub type SMTH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMMR_SPEC, u8, SMTH_A, 4, O>;
impl<'a, const O: u8> SMTH_W<'a, O> {
    #[doc = "1.9 V"]
    #[inline(always)]
    pub fn _1_9v(self) -> &'a mut W {
        self.variant(SMTH_A::_19v)
    }
    #[doc = "2.0 V"]
    #[inline(always)]
    pub fn _2_0v(self) -> &'a mut W {
        self.variant(SMTH_A::_20v)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn _2_1v(self) -> &'a mut W {
        self.variant(SMTH_A::_21v)
    }
    #[doc = "2.2 V"]
    #[inline(always)]
    pub fn _2_2v(self) -> &'a mut W {
        self.variant(SMTH_A::_22v)
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn _2_3v(self) -> &'a mut W {
        self.variant(SMTH_A::_23v)
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn _2_4v(self) -> &'a mut W {
        self.variant(SMTH_A::_24v)
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn _2_5v(self) -> &'a mut W {
        self.variant(SMTH_A::_25v)
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn _2_6v(self) -> &'a mut W {
        self.variant(SMTH_A::_26v)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn _2_7v(self) -> &'a mut W {
        self.variant(SMTH_A::_27v)
    }
    #[doc = "2.8 V"]
    #[inline(always)]
    pub fn _2_8v(self) -> &'a mut W {
        self.variant(SMTH_A::_28v)
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn _2_9v(self) -> &'a mut W {
        self.variant(SMTH_A::_29v)
    }
    #[doc = "3.0 V"]
    #[inline(always)]
    pub fn _3_0v(self) -> &'a mut W {
        self.variant(SMTH_A::_30v)
    }
    #[doc = "3.1 V"]
    #[inline(always)]
    pub fn _3_1v(self) -> &'a mut W {
        self.variant(SMTH_A::_31v)
    }
    #[doc = "3.2 V"]
    #[inline(always)]
    pub fn _3_2v(self) -> &'a mut W {
        self.variant(SMTH_A::_32v)
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn _3_3v(self) -> &'a mut W {
        self.variant(SMTH_A::_33v)
    }
    #[doc = "3.4 V"]
    #[inline(always)]
    pub fn _3_4v(self) -> &'a mut W {
        self.variant(SMTH_A::_34v)
    }
}
#[doc = "Field `SMSMPL` reader - Supply Monitor Sampling Period"]
pub type SMSMPL_R = crate::FieldReader<u8, SMSMPL_A>;
#[doc = "Supply Monitor Sampling Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMSMPL_A {
    #[doc = "0: Supply Monitor disabled"]
    Smd = 0,
    #[doc = "1: Continuous Supply Monitor"]
    Csm = 1,
    #[doc = "2: Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32slck = 2,
    #[doc = "3: Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256slck = 3,
    #[doc = "4: Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048slck = 4,
}
impl From<SMSMPL_A> for u8 {
    #[inline(always)]
    fn from(variant: SMSMPL_A) -> Self {
        variant as _
    }
}
impl SMSMPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMSMPL_A> {
        match self.bits {
            0 => Some(SMSMPL_A::Smd),
            1 => Some(SMSMPL_A::Csm),
            2 => Some(SMSMPL_A::_32slck),
            3 => Some(SMSMPL_A::_256slck),
            4 => Some(SMSMPL_A::_2048slck),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Smd`"]
    #[inline(always)]
    pub fn is_smd(&self) -> bool {
        *self == SMSMPL_A::Smd
    }
    #[doc = "Checks if the value of the field is `Csm`"]
    #[inline(always)]
    pub fn is_csm(&self) -> bool {
        *self == SMSMPL_A::Csm
    }
    #[doc = "Checks if the value of the field is `_32slck`"]
    #[inline(always)]
    pub fn is_32slck(&self) -> bool {
        *self == SMSMPL_A::_32slck
    }
    #[doc = "Checks if the value of the field is `_256slck`"]
    #[inline(always)]
    pub fn is_256slck(&self) -> bool {
        *self == SMSMPL_A::_256slck
    }
    #[doc = "Checks if the value of the field is `_2048slck`"]
    #[inline(always)]
    pub fn is_2048slck(&self) -> bool {
        *self == SMSMPL_A::_2048slck
    }
}
#[doc = "Field `SMSMPL` writer - Supply Monitor Sampling Period"]
pub type SMSMPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMMR_SPEC, u8, SMSMPL_A, 3, O>;
impl<'a, const O: u8> SMSMPL_W<'a, O> {
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn smd(self) -> &'a mut W {
        self.variant(SMSMPL_A::Smd)
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn csm(self) -> &'a mut W {
        self.variant(SMSMPL_A::Csm)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn _32slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_32slck)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn _256slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_256slck)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn _2048slck(self) -> &'a mut W {
        self.variant(SMSMPL_A::_2048slck)
    }
}
#[doc = "Field `SMRSTEN` reader - Supply Monitor Reset Enable"]
pub type SMRSTEN_R = crate::BitReader<SMRSTEN_A>;
#[doc = "Supply Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMRSTEN_A {
    #[doc = "0: the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    NotEnable = 0,
    #[doc = "1: the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    Enable = 1,
}
impl From<SMRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMRSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMRSTEN_A {
        match self.bits {
            false => SMRSTEN_A::NotEnable,
            true => SMRSTEN_A::Enable,
        }
    }
    #[doc = "Checks if the value of the field is `NotEnable`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMRSTEN_A::NotEnable
    }
    #[doc = "Checks if the value of the field is `Enable`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMRSTEN_A::Enable
    }
}
#[doc = "Field `SMRSTEN` writer - Supply Monitor Reset Enable"]
pub type SMRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMMR_SPEC, SMRSTEN_A, O>;
impl<'a, const O: u8> SMRSTEN_W<'a, O> {
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMRSTEN_A::NotEnable)
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMRSTEN_A::Enable)
    }
}
#[doc = "Field `SMIEN` reader - Supply Monitor Interrupt Enable"]
pub type SMIEN_R = crate::BitReader<SMIEN_A>;
#[doc = "Supply Monitor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMIEN_A {
    #[doc = "0: the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NotEnable = 0,
    #[doc = "1: the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    Enable = 1,
}
impl From<SMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMIEN_A {
        match self.bits {
            false => SMIEN_A::NotEnable,
            true => SMIEN_A::Enable,
        }
    }
    #[doc = "Checks if the value of the field is `NotEnable`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMIEN_A::NotEnable
    }
    #[doc = "Checks if the value of the field is `Enable`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMIEN_A::Enable
    }
}
#[doc = "Field `SMIEN` writer - Supply Monitor Interrupt Enable"]
pub type SMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMMR_SPEC, SMIEN_A, O>;
impl<'a, const O: u8> SMIEN_W<'a, O> {
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMIEN_A::NotEnable)
    }
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMIEN_A::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&self) -> SMTH_R {
        SMTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&self) -> SMSMPL_R {
        SMSMPL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&self) -> SMRSTEN_R {
        SMRSTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&self) -> SMIEN_R {
        SMIEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn smth(&mut self) -> SMTH_W<0> {
        SMTH_W::new(self)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    #[must_use]
    pub fn smsmpl(&mut self) -> SMSMPL_W<8> {
        SMSMPL_W::new(self)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smrsten(&mut self) -> SMRSTEN_W<12> {
        SMRSTEN_W::new(self)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smien(&mut self) -> SMIEN_W<13> {
        SMIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Supply Monitor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smmr](index.html) module"]
pub struct SMMR_SPEC;
impl crate::RegisterSpec for SMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smmr::R](R) reader structure"]
impl crate::Readable for SMMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smmr::W](W) writer structure"]
impl crate::Writable for SMMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMMR to value 0"]
impl crate::Resettable for SMMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
