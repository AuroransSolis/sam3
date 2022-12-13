#[doc = "Register `CR1` reader"]
#[derive(derive_more :: Deref, derive_more :: From)]
pub struct R(crate::R<CR1_SPEC>);
#[doc = "Register `CR1` writer"]
#[derive(derive_more :: Deref, derive_more :: DerefMut, derive_more :: From)]
pub struct W(crate::W<CR1_SPEC>);
#[doc = "Field `NC` reader - Number of Column Bits"]
pub type NC_R = crate::FieldReader<u8, NC_A>;
#[doc = "Number of Column Bits\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NC_A {
    #[doc = "0: 8 column bits"]
    Col8 = 0,
    #[doc = "1: 9 column bits"]
    Col9 = 1,
    #[doc = "2: 10 column bits"]
    Col10 = 2,
    #[doc = "3: 11 column bits"]
    Col11 = 3,
}
impl From<NC_A> for u8 {
    #[inline(always)]
    fn from(variant: NC_A) -> Self {
        variant as _
    }
}
impl NC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NC_A {
        match self.bits {
            0 => NC_A::Col8,
            1 => NC_A::Col9,
            2 => NC_A::Col10,
            3 => NC_A::Col11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Col8`"]
    #[inline(always)]
    pub fn is_col8(&self) -> bool {
        *self == NC_A::Col8
    }
    #[doc = "Checks if the value of the field is `Col9`"]
    #[inline(always)]
    pub fn is_col9(&self) -> bool {
        *self == NC_A::Col9
    }
    #[doc = "Checks if the value of the field is `Col10`"]
    #[inline(always)]
    pub fn is_col10(&self) -> bool {
        *self == NC_A::Col10
    }
    #[doc = "Checks if the value of the field is `Col11`"]
    #[inline(always)]
    pub fn is_col11(&self) -> bool {
        *self == NC_A::Col11
    }
}
#[doc = "Field `NC` writer - Number of Column Bits"]
pub type NC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, NC_A, 2, O>;
impl<'a, const O: u8> NC_W<'a, O> {
    #[doc = "8 column bits"]
    #[inline(always)]
    pub fn col8(self) -> &'a mut W {
        self.variant(NC_A::Col8)
    }
    #[doc = "9 column bits"]
    #[inline(always)]
    pub fn col9(self) -> &'a mut W {
        self.variant(NC_A::Col9)
    }
    #[doc = "10 column bits"]
    #[inline(always)]
    pub fn col10(self) -> &'a mut W {
        self.variant(NC_A::Col10)
    }
    #[doc = "11 column bits"]
    #[inline(always)]
    pub fn col11(self) -> &'a mut W {
        self.variant(NC_A::Col11)
    }
}
#[doc = "Field `NR` reader - Number of Row Bits"]
pub type NR_R = crate::FieldReader<u8, NR_A>;
#[doc = "Number of Row Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NR_A {
    #[doc = "0: 11 row bits"]
    Row11 = 0,
    #[doc = "1: 12 row bits"]
    Row12 = 1,
    #[doc = "2: 13 row bits"]
    Row13 = 2,
}
impl From<NR_A> for u8 {
    #[inline(always)]
    fn from(variant: NR_A) -> Self {
        variant as _
    }
}
impl NR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NR_A> {
        match self.bits {
            0 => Some(NR_A::Row11),
            1 => Some(NR_A::Row12),
            2 => Some(NR_A::Row13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Row11`"]
    #[inline(always)]
    pub fn is_row11(&self) -> bool {
        *self == NR_A::Row11
    }
    #[doc = "Checks if the value of the field is `Row12`"]
    #[inline(always)]
    pub fn is_row12(&self) -> bool {
        *self == NR_A::Row12
    }
    #[doc = "Checks if the value of the field is `Row13`"]
    #[inline(always)]
    pub fn is_row13(&self) -> bool {
        *self == NR_A::Row13
    }
}
#[doc = "Field `NR` writer - Number of Row Bits"]
pub type NR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, NR_A, 2, O>;
impl<'a, const O: u8> NR_W<'a, O> {
    #[doc = "11 row bits"]
    #[inline(always)]
    pub fn row11(self) -> &'a mut W {
        self.variant(NR_A::Row11)
    }
    #[doc = "12 row bits"]
    #[inline(always)]
    pub fn row12(self) -> &'a mut W {
        self.variant(NR_A::Row12)
    }
    #[doc = "13 row bits"]
    #[inline(always)]
    pub fn row13(self) -> &'a mut W {
        self.variant(NR_A::Row13)
    }
}
#[doc = "Field `NB` reader - Number of Banks"]
pub type NB_R = crate::BitReader<NB_A>;
#[doc = "Number of Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NB_A {
    #[doc = "0: 2 banks"]
    Bank2 = 0,
    #[doc = "1: 4 banks"]
    Bank4 = 1,
}
impl From<NB_A> for bool {
    #[inline(always)]
    fn from(variant: NB_A) -> Self {
        variant as u8 != 0
    }
}
impl NB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NB_A {
        match self.bits {
            false => NB_A::Bank2,
            true => NB_A::Bank4,
        }
    }
    #[doc = "Checks if the value of the field is `Bank2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == NB_A::Bank2
    }
    #[doc = "Checks if the value of the field is `Bank4`"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == NB_A::Bank4
    }
}
#[doc = "Field `NB` writer - Number of Banks"]
pub type NB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, NB_A, O>;
impl<'a, const O: u8> NB_W<'a, O> {
    #[doc = "2 banks"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(NB_A::Bank2)
    }
    #[doc = "4 banks"]
    #[inline(always)]
    pub fn bank4(self) -> &'a mut W {
        self.variant(NB_A::Bank4)
    }
}
#[doc = "Field `CAS` reader - CAS Latency"]
pub type CAS_R = crate::FieldReader<u8, CAS_A>;
#[doc = "CAS Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAS_A {
    #[doc = "1: 1 cycle CAS latency"]
    Latency1 = 1,
    #[doc = "2: 2 cycle CAS latency"]
    Latency2 = 2,
    #[doc = "3: 3 cycle CAS latency"]
    Latency3 = 3,
}
impl From<CAS_A> for u8 {
    #[inline(always)]
    fn from(variant: CAS_A) -> Self {
        variant as _
    }
}
impl CAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAS_A> {
        match self.bits {
            1 => Some(CAS_A::Latency1),
            2 => Some(CAS_A::Latency2),
            3 => Some(CAS_A::Latency3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Latency1`"]
    #[inline(always)]
    pub fn is_latency1(&self) -> bool {
        *self == CAS_A::Latency1
    }
    #[doc = "Checks if the value of the field is `Latency2`"]
    #[inline(always)]
    pub fn is_latency2(&self) -> bool {
        *self == CAS_A::Latency2
    }
    #[doc = "Checks if the value of the field is `Latency3`"]
    #[inline(always)]
    pub fn is_latency3(&self) -> bool {
        *self == CAS_A::Latency3
    }
}
#[doc = "Field `CAS` writer - CAS Latency"]
pub type CAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, CAS_A, 2, O>;
impl<'a, const O: u8> CAS_W<'a, O> {
    #[doc = "1 cycle CAS latency"]
    #[inline(always)]
    pub fn latency1(self) -> &'a mut W {
        self.variant(CAS_A::Latency1)
    }
    #[doc = "2 cycle CAS latency"]
    #[inline(always)]
    pub fn latency2(self) -> &'a mut W {
        self.variant(CAS_A::Latency2)
    }
    #[doc = "3 cycle CAS latency"]
    #[inline(always)]
    pub fn latency3(self) -> &'a mut W {
        self.variant(CAS_A::Latency3)
    }
}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DBW_R = crate::BitReader<bool>;
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DBW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `TWR` reader - Write Recovery Delay"]
pub type TWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWR` writer - Write Recovery Delay"]
pub type TWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRC_TRFC` reader - Row Cycle Delay and Row Refresh Cycle"]
pub type TRC_TRFC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRC_TRFC` writer - Row Cycle Delay and Row Refresh Cycle"]
pub type TRC_TRFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRP` reader - Row Precharge Delay"]
pub type TRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRP` writer - Row Precharge Delay"]
pub type TRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRCD` reader - Row to Column Delay"]
pub type TRCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRCD` writer - Row to Column Delay"]
pub type TRCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRAS` reader - Active to Precharge Delay"]
pub type TRAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRAS` writer - Active to Precharge Delay"]
pub type TRAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXSR` reader - Exit Self Refresh to Active Delay"]
pub type TXSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXSR` writer - Exit Self Refresh to Active Delay"]
pub type TXSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    pub fn trc_trfc(&self) -> TRC_TRFC_R {
        TRC_TRFC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Exit Self Refresh to Active Delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    #[must_use]
    pub fn nc(&mut self) -> NC_W<0> {
        NC_W::new(self)
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    #[must_use]
    pub fn nr(&mut self) -> NR_W<2> {
        NR_W::new(self)
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline(always)]
    #[must_use]
    pub fn nb(&mut self) -> NB_W<4> {
        NB_W::new(self)
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CAS_W<5> {
        CAS_W::new(self)
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn dbw(&mut self) -> DBW_W<7> {
        DBW_W::new(self)
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    #[must_use]
    pub fn twr(&mut self) -> TWR_W<8> {
        TWR_W::new(self)
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn trc_trfc(&mut self) -> TRC_TRFC_W<12> {
        TRC_TRFC_W::new(self)
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    #[must_use]
    pub fn trp(&mut self) -> TRP_W<16> {
        TRP_W::new(self)
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline(always)]
    #[must_use]
    pub fn trcd(&mut self) -> TRCD_W<20> {
        TRCD_W::new(self)
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tras(&mut self) -> TRAS_W<24> {
        TRAS_W::new(self)
    }
    #[doc = "Bits 28:31 - Exit Self Refresh to Active Delay"]
    #[inline(always)]
    #[must_use]
    pub fn txsr(&mut self) -> TXSR_W<28> {
        TXSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0x02"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
