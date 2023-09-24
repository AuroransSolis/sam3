#[doc = "Register `MODE6` reader"]
pub type R = crate::R<MODE6_SPEC>;
#[doc = "Register `MODE6` writer"]
pub type W = crate::W<MODE6_SPEC>;
#[doc = "Field `READ_MODE` reader - Selection of the Control Signal for Read Operation"]
pub type READ_MODE_R = crate::BitReader<READ_MODE_A>;
#[doc = "Selection of the Control Signal for Read Operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READ_MODE_A {
    #[doc = "0: The Read operation is controlled by the NCS signal."]
    NcsCtrl = 0,
    #[doc = "1: The Read operation is controlled by the NRD signal."]
    NrdCtrl = 1,
}
impl From<READ_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_MODE_A {
        match self.bits {
            false => READ_MODE_A::NcsCtrl,
            true => READ_MODE_A::NrdCtrl,
        }
    }
    #[doc = "The Read operation is controlled by the NCS signal."]
    #[inline(always)]
    pub fn is_ncs_ctrl(&self) -> bool {
        *self == READ_MODE_A::NcsCtrl
    }
    #[doc = "The Read operation is controlled by the NRD signal."]
    #[inline(always)]
    pub fn is_nrd_ctrl(&self) -> bool {
        *self == READ_MODE_A::NrdCtrl
    }
}
#[doc = "Field `READ_MODE` writer - Selection of the Control Signal for Read Operation"]
pub type READ_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, READ_MODE_A>;
impl<'a, REG, const O: u8> READ_MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Read operation is controlled by the NCS signal."]
    #[inline(always)]
    pub fn ncs_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(READ_MODE_A::NcsCtrl)
    }
    #[doc = "The Read operation is controlled by the NRD signal."]
    #[inline(always)]
    pub fn nrd_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(READ_MODE_A::NrdCtrl)
    }
}
#[doc = "Field `WRITE_MODE` reader - Selection of the Control Signal for Write Operation"]
pub type WRITE_MODE_R = crate::BitReader<WRITE_MODE_A>;
#[doc = "Selection of the Control Signal for Write Operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRITE_MODE_A {
    #[doc = "0: The Write operation is controller by the NCS signal."]
    NcsCtrl = 0,
    #[doc = "1: The Write operation is controlled by the NWE signal."]
    NweCtrl = 1,
}
impl From<WRITE_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_MODE_A {
        match self.bits {
            false => WRITE_MODE_A::NcsCtrl,
            true => WRITE_MODE_A::NweCtrl,
        }
    }
    #[doc = "The Write operation is controller by the NCS signal."]
    #[inline(always)]
    pub fn is_ncs_ctrl(&self) -> bool {
        *self == WRITE_MODE_A::NcsCtrl
    }
    #[doc = "The Write operation is controlled by the NWE signal."]
    #[inline(always)]
    pub fn is_nwe_ctrl(&self) -> bool {
        *self == WRITE_MODE_A::NweCtrl
    }
}
#[doc = "Field `WRITE_MODE` writer - Selection of the Control Signal for Write Operation"]
pub type WRITE_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WRITE_MODE_A>;
impl<'a, REG, const O: u8> WRITE_MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Write operation is controller by the NCS signal."]
    #[inline(always)]
    pub fn ncs_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(WRITE_MODE_A::NcsCtrl)
    }
    #[doc = "The Write operation is controlled by the NWE signal."]
    #[inline(always)]
    pub fn nwe_ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(WRITE_MODE_A::NweCtrl)
    }
}
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub type EXNW_MODE_R = crate::FieldReader<EXNW_MODE_A>;
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXNW_MODE_A {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "2: Frozen Mode"]
    Frozen = 2,
    #[doc = "3: Ready Mode"]
    Ready = 3,
}
impl From<EXNW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXNW_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXNW_MODE_A {
    type Ux = u8;
}
impl EXNW_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXNW_MODE_A> {
        match self.bits {
            0 => Some(EXNW_MODE_A::Disabled),
            2 => Some(EXNW_MODE_A::Frozen),
            3 => Some(EXNW_MODE_A::Ready),
            _ => None,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXNW_MODE_A::Disabled
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == EXNW_MODE_A::Frozen
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXNW_MODE_A::Ready
    }
}
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub type EXNW_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, EXNW_MODE_A>;
impl<'a, REG, const O: u8> EXNW_MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXNW_MODE_A::Disabled)
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(EXNW_MODE_A::Frozen)
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(EXNW_MODE_A::Ready)
    }
}
#[doc = "Field `BAT` reader - Byte Access Type"]
pub type BAT_R = crate::BitReader;
#[doc = "Field `BAT` writer - Byte Access Type"]
pub type BAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DBW_R = crate::BitReader<DBW_A>;
#[doc = "Data Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBW_A {
    #[doc = "0: 8-bit bus"]
    Bit8 = 0,
    #[doc = "1: 16-bit bus"]
    Bit16 = 1,
}
impl From<DBW_A> for bool {
    #[inline(always)]
    fn from(variant: DBW_A) -> Self {
        variant as u8 != 0
    }
}
impl DBW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBW_A {
        match self.bits {
            false => DBW_A::Bit8,
            true => DBW_A::Bit16,
        }
    }
    #[doc = "8-bit bus"]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == DBW_A::Bit8
    }
    #[doc = "16-bit bus"]
    #[inline(always)]
    pub fn is_bit_16(&self) -> bool {
        *self == DBW_A::Bit16
    }
}
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DBW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DBW_A>;
impl<'a, REG, const O: u8> DBW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit bus"]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut crate::W<REG> {
        self.variant(DBW_A::Bit8)
    }
    #[doc = "16-bit bus"]
    #[inline(always)]
    pub fn bit_16(self) -> &'a mut crate::W<REG> {
        self.variant(DBW_A::Bit16)
    }
}
#[doc = "Field `TDF_CYCLES` reader - Data Float Time"]
pub type TDF_CYCLES_R = crate::FieldReader;
#[doc = "Field `TDF_CYCLES` writer - Data Float Time"]
pub type TDF_CYCLES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TDF_MODE` reader - TDF Optimization"]
pub type TDF_MODE_R = crate::BitReader;
#[doc = "Field `TDF_MODE` writer - TDF Optimization"]
pub type TDF_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Selection of the Control Signal for Read Operation"]
    #[inline(always)]
    pub fn read_mode(&self) -> READ_MODE_R {
        READ_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selection of the Control Signal for Write Operation"]
    #[inline(always)]
    pub fn write_mode(&self) -> WRITE_MODE_R {
        WRITE_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> EXNW_MODE_R {
        EXNW_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&self) -> BAT_R {
        BAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TDF_CYCLES_R {
        TDF_CYCLES_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TDF_MODE_R {
        TDF_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selection of the Control Signal for Read Operation"]
    #[inline(always)]
    #[must_use]
    pub fn read_mode(&mut self) -> READ_MODE_W<MODE6_SPEC, 0> {
        READ_MODE_W::new(self)
    }
    #[doc = "Bit 1 - Selection of the Control Signal for Write Operation"]
    #[inline(always)]
    #[must_use]
    pub fn write_mode(&mut self) -> WRITE_MODE_W<MODE6_SPEC, 1> {
        WRITE_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn exnw_mode(&mut self) -> EXNW_MODE_W<MODE6_SPEC, 4> {
        EXNW_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    #[must_use]
    pub fn bat(&mut self) -> BAT_W<MODE6_SPEC, 8> {
        BAT_W::new(self)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn dbw(&mut self) -> DBW_W<MODE6_SPEC, 12> {
        DBW_W::new(self)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    #[must_use]
    pub fn tdf_cycles(&mut self) -> TDF_CYCLES_W<MODE6_SPEC, 16> {
        TDF_CYCLES_W::new(self)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    #[must_use]
    pub fn tdf_mode(&mut self) -> TDF_MODE_W<MODE6_SPEC, 20> {
        TDF_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SMC Mode Register (CS_number = 6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE6_SPEC;
impl crate::RegisterSpec for MODE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode6::R`](R) reader structure"]
impl crate::Readable for MODE6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode6::W`](W) writer structure"]
impl crate::Writable for MODE6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE6 to value 0x1000_0003"]
impl crate::Resettable for MODE6_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0003;
}
