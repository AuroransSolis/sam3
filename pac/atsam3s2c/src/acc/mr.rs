#[doc = "Register `MR` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<MR_SPEC>);
#[doc = "Register `MR` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<MR_SPEC>);
#[doc = "Field `SELMINUS` reader - SELection for MINUS comparator input"]
pub type SELMINUS_R = crate::FieldReader<u8, SELMINUS_A>;
#[doc = "SELection for MINUS comparator input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELMINUS_A {
    #[doc = "0: SelectTS"]
    Ts = 0,
    #[doc = "1: Select ADVREF"]
    Advref = 1,
    #[doc = "2: Select DAC0"]
    Dac0 = 2,
    #[doc = "3: Select DAC1"]
    Dac1 = 3,
    #[doc = "4: Select AD0"]
    Ad0 = 4,
    #[doc = "5: Select AD1"]
    Ad1 = 5,
    #[doc = "6: Select AD2"]
    Ad2 = 6,
    #[doc = "7: Select AD3"]
    Ad3 = 7,
}
impl From<SELMINUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELMINUS_A) -> Self {
        variant as _
    }
}
impl SELMINUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELMINUS_A {
        match self.bits {
            0 => SELMINUS_A::Ts,
            1 => SELMINUS_A::Advref,
            2 => SELMINUS_A::Dac0,
            3 => SELMINUS_A::Dac1,
            4 => SELMINUS_A::Ad0,
            5 => SELMINUS_A::Ad1,
            6 => SELMINUS_A::Ad2,
            7 => SELMINUS_A::Ad3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Ts`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == SELMINUS_A::Ts
    }
    #[doc = "Checks if the value of the field is `Advref`"]
    #[inline(always)]
    pub fn is_advref(&self) -> bool {
        *self == SELMINUS_A::Advref
    }
    #[doc = "Checks if the value of the field is `Dac0`"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == SELMINUS_A::Dac0
    }
    #[doc = "Checks if the value of the field is `Dac1`"]
    #[inline(always)]
    pub fn is_dac1(&self) -> bool {
        *self == SELMINUS_A::Dac1
    }
    #[doc = "Checks if the value of the field is `Ad0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == SELMINUS_A::Ad0
    }
    #[doc = "Checks if the value of the field is `Ad1`"]
    #[inline(always)]
    pub fn is_ad1(&self) -> bool {
        *self == SELMINUS_A::Ad1
    }
    #[doc = "Checks if the value of the field is `Ad2`"]
    #[inline(always)]
    pub fn is_ad2(&self) -> bool {
        *self == SELMINUS_A::Ad2
    }
    #[doc = "Checks if the value of the field is `Ad3`"]
    #[inline(always)]
    pub fn is_ad3(&self) -> bool {
        *self == SELMINUS_A::Ad3
    }
}
#[doc = "Field `SELMINUS` writer - SELection for MINUS comparator input"]
pub type SELMINUS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MR_SPEC, u8, SELMINUS_A, 3, O>;
impl<'a, const O: u8> SELMINUS_W<'a, O> {
    #[doc = "SelectTS"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(SELMINUS_A::Ts)
    }
    #[doc = "Select ADVREF"]
    #[inline(always)]
    pub fn advref(self) -> &'a mut W {
        self.variant(SELMINUS_A::Advref)
    }
    #[doc = "Select DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut W {
        self.variant(SELMINUS_A::Dac0)
    }
    #[doc = "Select DAC1"]
    #[inline(always)]
    pub fn dac1(self) -> &'a mut W {
        self.variant(SELMINUS_A::Dac1)
    }
    #[doc = "Select AD0"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(SELMINUS_A::Ad0)
    }
    #[doc = "Select AD1"]
    #[inline(always)]
    pub fn ad1(self) -> &'a mut W {
        self.variant(SELMINUS_A::Ad1)
    }
    #[doc = "Select AD2"]
    #[inline(always)]
    pub fn ad2(self) -> &'a mut W {
        self.variant(SELMINUS_A::Ad2)
    }
    #[doc = "Select AD3"]
    #[inline(always)]
    pub fn ad3(self) -> &'a mut W {
        self.variant(SELMINUS_A::Ad3)
    }
}
#[doc = "Field `SELPLUS` reader - SELection for PLUS comparator input"]
pub type SELPLUS_R = crate::FieldReader<u8, SELPLUS_A>;
#[doc = "SELection for PLUS comparator input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELPLUS_A {
    #[doc = "0: Select AD0"]
    Ad0 = 0,
    #[doc = "1: Select AD1"]
    Ad1 = 1,
    #[doc = "2: Select AD2"]
    Ad2 = 2,
    #[doc = "3: Select AD3"]
    Ad3 = 3,
    #[doc = "4: Select AD4"]
    Ad4 = 4,
    #[doc = "5: Select AD5"]
    Ad5 = 5,
    #[doc = "6: Select AD6"]
    Ad6 = 6,
    #[doc = "7: Select AD7"]
    Ad7 = 7,
}
impl From<SELPLUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELPLUS_A) -> Self {
        variant as _
    }
}
impl SELPLUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELPLUS_A {
        match self.bits {
            0 => SELPLUS_A::Ad0,
            1 => SELPLUS_A::Ad1,
            2 => SELPLUS_A::Ad2,
            3 => SELPLUS_A::Ad3,
            4 => SELPLUS_A::Ad4,
            5 => SELPLUS_A::Ad5,
            6 => SELPLUS_A::Ad6,
            7 => SELPLUS_A::Ad7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Ad0`"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == SELPLUS_A::Ad0
    }
    #[doc = "Checks if the value of the field is `Ad1`"]
    #[inline(always)]
    pub fn is_ad1(&self) -> bool {
        *self == SELPLUS_A::Ad1
    }
    #[doc = "Checks if the value of the field is `Ad2`"]
    #[inline(always)]
    pub fn is_ad2(&self) -> bool {
        *self == SELPLUS_A::Ad2
    }
    #[doc = "Checks if the value of the field is `Ad3`"]
    #[inline(always)]
    pub fn is_ad3(&self) -> bool {
        *self == SELPLUS_A::Ad3
    }
    #[doc = "Checks if the value of the field is `Ad4`"]
    #[inline(always)]
    pub fn is_ad4(&self) -> bool {
        *self == SELPLUS_A::Ad4
    }
    #[doc = "Checks if the value of the field is `Ad5`"]
    #[inline(always)]
    pub fn is_ad5(&self) -> bool {
        *self == SELPLUS_A::Ad5
    }
    #[doc = "Checks if the value of the field is `Ad6`"]
    #[inline(always)]
    pub fn is_ad6(&self) -> bool {
        *self == SELPLUS_A::Ad6
    }
    #[doc = "Checks if the value of the field is `Ad7`"]
    #[inline(always)]
    pub fn is_ad7(&self) -> bool {
        *self == SELPLUS_A::Ad7
    }
}
#[doc = "Field `SELPLUS` writer - SELection for PLUS comparator input"]
pub type SELPLUS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MR_SPEC, u8, SELPLUS_A, 3, O>;
impl<'a, const O: u8> SELPLUS_W<'a, O> {
    #[doc = "Select AD0"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut W {
        self.variant(SELPLUS_A::Ad0)
    }
    #[doc = "Select AD1"]
    #[inline(always)]
    pub fn ad1(self) -> &'a mut W {
        self.variant(SELPLUS_A::Ad1)
    }
    #[doc = "Select AD2"]
    #[inline(always)]
    pub fn ad2(self) -> &'a mut W {
        self.variant(SELPLUS_A::Ad2)
    }
    #[doc = "Select AD3"]
    #[inline(always)]
    pub fn ad3(self) -> &'a mut W {
        self.variant(SELPLUS_A::Ad3)
    }
    #[doc = "Select AD4"]
    #[inline(always)]
    pub fn ad4(self) -> &'a mut W {
        self.variant(SELPLUS_A::Ad4)
    }
    #[doc = "Select AD5"]
    #[inline(always)]
    pub fn ad5(self) -> &'a mut W {
        self.variant(SELPLUS_A::Ad5)
    }
    #[doc = "Select AD6"]
    #[inline(always)]
    pub fn ad6(self) -> &'a mut W {
        self.variant(SELPLUS_A::Ad6)
    }
    #[doc = "Select AD7"]
    #[inline(always)]
    pub fn ad7(self) -> &'a mut W {
        self.variant(SELPLUS_A::Ad7)
    }
}
#[doc = "Field `ACEN` reader - Analog Comparator ENable"]
pub type ACEN_R = crate::BitReader<ACEN_A>;
#[doc = "Analog Comparator ENable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACEN_A {
    #[doc = "0: Analog Comparator Disabled."]
    Dis = 0,
    #[doc = "1: Analog Comparator Enabled."]
    En = 1,
}
impl From<ACEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACEN_A {
        match self.bits {
            false => ACEN_A::Dis,
            true => ACEN_A::En,
        }
    }
    #[doc = "Checks if the value of the field is `Dis`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACEN_A::Dis
    }
    #[doc = "Checks if the value of the field is `En`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ACEN_A::En
    }
}
#[doc = "Field `ACEN` writer - Analog Comparator ENable"]
pub type ACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, ACEN_A, O>;
impl<'a, const O: u8> ACEN_W<'a, O> {
    #[doc = "Analog Comparator Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACEN_A::Dis)
    }
    #[doc = "Analog Comparator Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ACEN_A::En)
    }
}
#[doc = "Field `EDGETYP` reader - EDGE TYPe"]
pub type EDGETYP_R = crate::FieldReader<u8, EDGETYP_A>;
#[doc = "EDGE TYPe\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGETYP_A {
    #[doc = "0: only rising edge of comparator output"]
    Rising = 0,
    #[doc = "1: falling edge of comparator output"]
    Falling = 1,
    #[doc = "2: any edge of comparator output"]
    Any = 2,
}
impl From<EDGETYP_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGETYP_A) -> Self {
        variant as _
    }
}
impl EDGETYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EDGETYP_A> {
        match self.bits {
            0 => Some(EDGETYP_A::Rising),
            1 => Some(EDGETYP_A::Falling),
            2 => Some(EDGETYP_A::Any),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Rising`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EDGETYP_A::Rising
    }
    #[doc = "Checks if the value of the field is `Falling`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EDGETYP_A::Falling
    }
    #[doc = "Checks if the value of the field is `Any`"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == EDGETYP_A::Any
    }
}
#[doc = "Field `EDGETYP` writer - EDGE TYPe"]
pub type EDGETYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, EDGETYP_A, 2, O>;
impl<'a, const O: u8> EDGETYP_W<'a, O> {
    #[doc = "only rising edge of comparator output"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGETYP_A::Rising)
    }
    #[doc = "falling edge of comparator output"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGETYP_A::Falling)
    }
    #[doc = "any edge of comparator output"]
    #[inline(always)]
    pub fn any(self) -> &'a mut W {
        self.variant(EDGETYP_A::Any)
    }
}
#[doc = "Field `INV` reader - INVert comparator output"]
pub type INV_R = crate::BitReader<INV_A>;
#[doc = "INVert comparator output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    #[doc = "0: Analog Comparator output is directly processed."]
    Dis = 0,
    #[doc = "1: Analog Comparator output is inverted prior to being processed."]
    En = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
impl INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::Dis,
            true => INV_A::En,
        }
    }
    #[doc = "Checks if the value of the field is `Dis`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == INV_A::Dis
    }
    #[doc = "Checks if the value of the field is `En`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == INV_A::En
    }
}
#[doc = "Field `INV` writer - INVert comparator output"]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, INV_A, O>;
impl<'a, const O: u8> INV_W<'a, O> {
    #[doc = "Analog Comparator output is directly processed."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INV_A::Dis)
    }
    #[doc = "Analog Comparator output is inverted prior to being processed."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(INV_A::En)
    }
}
#[doc = "Field `SELFS` reader - SELection of Fault Source"]
pub type SELFS_R = crate::BitReader<SELFS_A>;
#[doc = "SELection of Fault Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELFS_A {
    #[doc = "0: the CF flag is used to drive the FAULT output."]
    Cf = 0,
    #[doc = "1: the output of the Analog Comparator flag is used to drive the FAULT output."]
    Output = 1,
}
impl From<SELFS_A> for bool {
    #[inline(always)]
    fn from(variant: SELFS_A) -> Self {
        variant as u8 != 0
    }
}
impl SELFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELFS_A {
        match self.bits {
            false => SELFS_A::Cf,
            true => SELFS_A::Output,
        }
    }
    #[doc = "Checks if the value of the field is `Cf`"]
    #[inline(always)]
    pub fn is_cf(&self) -> bool {
        *self == SELFS_A::Cf
    }
    #[doc = "Checks if the value of the field is `Output`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == SELFS_A::Output
    }
}
#[doc = "Field `SELFS` writer - SELection of Fault Source"]
pub type SELFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, SELFS_A, O>;
impl<'a, const O: u8> SELFS_W<'a, O> {
    #[doc = "the CF flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn cf(self) -> &'a mut W {
        self.variant(SELFS_A::Cf)
    }
    #[doc = "the output of the Analog Comparator flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(SELFS_A::Output)
    }
}
#[doc = "Field `FE` reader - Fault Enable"]
pub type FE_R = crate::BitReader<FE_A>;
#[doc = "Fault Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_A {
    #[doc = "0: the FAULT output is tied to 0."]
    Dis = 0,
    #[doc = "1: the FAULT output is driven by the signal defined by SELFS."]
    En = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
impl FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::Dis,
            true => FE_A::En,
        }
    }
    #[doc = "Checks if the value of the field is `Dis`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FE_A::Dis
    }
    #[doc = "Checks if the value of the field is `En`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FE_A::En
    }
}
#[doc = "Field `FE` writer - Fault Enable"]
pub type FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, FE_A, O>;
impl<'a, const O: u8> FE_W<'a, O> {
    #[doc = "the FAULT output is tied to 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FE_A::Dis)
    }
    #[doc = "the FAULT output is driven by the signal defined by SELFS."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FE_A::En)
    }
}
impl R {
    #[doc = "Bits 0:2 - SELection for MINUS comparator input"]
    #[inline(always)]
    pub fn selminus(&self) -> SELMINUS_R {
        SELMINUS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - SELection for PLUS comparator input"]
    #[inline(always)]
    pub fn selplus(&self) -> SELPLUS_R {
        SELPLUS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Analog Comparator ENable"]
    #[inline(always)]
    pub fn acen(&self) -> ACEN_R {
        ACEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - EDGE TYPe"]
    #[inline(always)]
    pub fn edgetyp(&self) -> EDGETYP_R {
        EDGETYP_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12 - INVert comparator output"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SELection of Fault Source"]
    #[inline(always)]
    pub fn selfs(&self) -> SELFS_R {
        SELFS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SELection for MINUS comparator input"]
    #[inline(always)]
    #[must_use]
    pub fn selminus(&mut self) -> SELMINUS_W<0> {
        SELMINUS_W::new(self)
    }
    #[doc = "Bits 4:6 - SELection for PLUS comparator input"]
    #[inline(always)]
    #[must_use]
    pub fn selplus(&mut self) -> SELPLUS_W<4> {
        SELPLUS_W::new(self)
    }
    #[doc = "Bit 8 - Analog Comparator ENable"]
    #[inline(always)]
    #[must_use]
    pub fn acen(&mut self) -> ACEN_W<8> {
        ACEN_W::new(self)
    }
    #[doc = "Bits 9:10 - EDGE TYPe"]
    #[inline(always)]
    #[must_use]
    pub fn edgetyp(&mut self) -> EDGETYP_W<9> {
        EDGETYP_W::new(self)
    }
    #[doc = "Bit 12 - INVert comparator output"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<12> {
        INV_W::new(self)
    }
    #[doc = "Bit 13 - SELection of Fault Source"]
    #[inline(always)]
    #[must_use]
    pub fn selfs(&mut self) -> SELFS_W<13> {
        SELFS_W::new(self)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<14> {
        FE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
