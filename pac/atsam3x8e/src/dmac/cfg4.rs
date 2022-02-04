#[doc = "Register `CFG4` reader"]
pub struct R(crate::R<CFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG4` writer"]
pub struct W(crate::W<CFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG4_SPEC>;
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
impl From<crate::W<CFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC_PER` reader - Source with Peripheral identifier"]
pub struct SRC_PER_R(crate::FieldReader<u8, u8>);
impl SRC_PER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRC_PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC_PER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_PER` writer - Source with Peripheral identifier"]
pub struct SRC_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `DST_PER` reader - Destination with Peripheral identifier"]
pub struct DST_PER_R(crate::FieldReader<u8, u8>);
impl DST_PER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DST_PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DST_PER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_PER` writer - Destination with Peripheral identifier"]
pub struct DST_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Software or Hardware Selection for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_H2SEL_A {
    #[doc = "0: Software handshaking interface is used to trigger a transfer request."]
    SW = 0,
    #[doc = "1: Hardware handshaking interface is used to trigger a transfer request."]
    HW = 1,
}
impl From<SRC_H2SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_H2SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC_H2SEL` reader - Software or Hardware Selection for the Source"]
pub struct SRC_H2SEL_R(crate::FieldReader<bool, SRC_H2SEL_A>);
impl SRC_H2SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRC_H2SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_H2SEL_A {
        match self.bits {
            false => SRC_H2SEL_A::SW,
            true => SRC_H2SEL_A::HW,
        }
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        **self == SRC_H2SEL_A::SW
    }
    #[doc = "Checks if the value of the field is `HW`"]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        **self == SRC_H2SEL_A::HW
    }
}
impl core::ops::Deref for SRC_H2SEL_R {
    type Target = crate::FieldReader<bool, SRC_H2SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC_H2SEL` writer - Software or Hardware Selection for the Source"]
pub struct SRC_H2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_H2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_H2SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(SRC_H2SEL_A::SW)
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn hw(self) -> &'a mut W {
        self.variant(SRC_H2SEL_A::HW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Software or Hardware Selection for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_H2SEL_A {
    #[doc = "0: Software handshaking interface is used to trigger a transfer request."]
    SW = 0,
    #[doc = "1: Hardware handshaking interface is used to trigger a transfer request."]
    HW = 1,
}
impl From<DST_H2SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DST_H2SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DST_H2SEL` reader - Software or Hardware Selection for the Destination"]
pub struct DST_H2SEL_R(crate::FieldReader<bool, DST_H2SEL_A>);
impl DST_H2SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DST_H2SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_H2SEL_A {
        match self.bits {
            false => DST_H2SEL_A::SW,
            true => DST_H2SEL_A::HW,
        }
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        **self == DST_H2SEL_A::SW
    }
    #[doc = "Checks if the value of the field is `HW`"]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        **self == DST_H2SEL_A::HW
    }
}
impl core::ops::Deref for DST_H2SEL_R {
    type Target = crate::FieldReader<bool, DST_H2SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_H2SEL` writer - Software or Hardware Selection for the Destination"]
pub struct DST_H2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_H2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_H2SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(DST_H2SEL_A::SW)
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline(always)]
    pub fn hw(self) -> &'a mut W {
        self.variant(DST_H2SEL_A::HW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Stop On Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOD_A {
    #[doc = "0: STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    DISABLE = 0,
    #[doc = "1: STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    ENABLE = 1,
}
impl From<SOD_A> for bool {
    #[inline(always)]
    fn from(variant: SOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOD` reader - Stop On Done"]
pub struct SOD_R(crate::FieldReader<bool, SOD_A>);
impl SOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOD_A {
        match self.bits {
            false => SOD_A::DISABLE,
            true => SOD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SOD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SOD_A::ENABLE
    }
}
impl core::ops::Deref for SOD_R {
    type Target = crate::FieldReader<bool, SOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOD` writer - Stop On Done"]
pub struct SOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SOD_A::DISABLE)
    }
    #[doc = "STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SOD_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Interface Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_IF_A {
    #[doc = "0: Interface Lock capability is disabled"]
    DISABLE = 0,
    #[doc = "1: Interface Lock capability is enabled"]
    ENABLE = 1,
}
impl From<LOCK_IF_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_IF` reader - Interface Lock"]
pub struct LOCK_IF_R(crate::FieldReader<bool, LOCK_IF_A>);
impl LOCK_IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_IF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_IF_A {
        match self.bits {
            false => LOCK_IF_A::DISABLE,
            true => LOCK_IF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == LOCK_IF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LOCK_IF_A::ENABLE
    }
}
impl core::ops::Deref for LOCK_IF_R {
    type Target = crate::FieldReader<bool, LOCK_IF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_IF` writer - Interface Lock"]
pub struct LOCK_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_IF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_IF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interface Lock capability is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOCK_IF_A::DISABLE)
    }
    #[doc = "Interface Lock capability is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOCK_IF_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Bus Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_B_A {
    #[doc = "0: AHB Bus Locking capability is disabled."]
    DISABLE = 0,
}
impl From<LOCK_B_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_B` reader - Bus Lock"]
pub struct LOCK_B_R(crate::FieldReader<bool, LOCK_B_A>);
impl LOCK_B_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_B_A> {
        match self.bits {
            false => Some(LOCK_B_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == LOCK_B_A::DISABLE
    }
}
impl core::ops::Deref for LOCK_B_R {
    type Target = crate::FieldReader<bool, LOCK_B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_B` writer - Bus Lock"]
pub struct LOCK_B_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_B_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "AHB Bus Locking capability is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOCK_B_A::DISABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Master Interface Arbiter Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_IF_L_A {
    #[doc = "0: The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    CHUNK = 0,
    #[doc = "1: The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    BUFFER = 1,
}
impl From<LOCK_IF_L_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_IF_L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_IF_L` reader - Master Interface Arbiter Lock"]
pub struct LOCK_IF_L_R(crate::FieldReader<bool, LOCK_IF_L_A>);
impl LOCK_IF_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_IF_L_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_IF_L_A {
        match self.bits {
            false => LOCK_IF_L_A::CHUNK,
            true => LOCK_IF_L_A::BUFFER,
        }
    }
    #[doc = "Checks if the value of the field is `CHUNK`"]
    #[inline(always)]
    pub fn is_chunk(&self) -> bool {
        **self == LOCK_IF_L_A::CHUNK
    }
    #[doc = "Checks if the value of the field is `BUFFER`"]
    #[inline(always)]
    pub fn is_buffer(&self) -> bool {
        **self == LOCK_IF_L_A::BUFFER
    }
}
impl core::ops::Deref for LOCK_IF_L_R {
    type Target = crate::FieldReader<bool, LOCK_IF_L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_IF_L` writer - Master Interface Arbiter Lock"]
pub struct LOCK_IF_L_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_IF_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_IF_L_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    #[inline(always)]
    pub fn chunk(self) -> &'a mut W {
        self.variant(LOCK_IF_L_A::CHUNK)
    }
    #[doc = "The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    #[inline(always)]
    pub fn buffer(self) -> &'a mut W {
        self.variant(LOCK_IF_L_A::BUFFER)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `AHB_PROT` reader - AHB Protection"]
pub struct AHB_PROT_R(crate::FieldReader<u8, u8>);
impl AHB_PROT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AHB_PROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_PROT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_PROT` writer - AHB Protection"]
pub struct AHB_PROT_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_PROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "FIFO Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIFOCFG_A {
    #[doc = "0: The largest defined length AHB burst is performed on the destination AHB interface."]
    ALAP_CFG = 0,
    #[doc = "1: When half FIFO size is available/filled, a source/destination request is serviced."]
    HALF_CFG = 1,
    #[doc = "2: When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    ASAP_CFG = 2,
}
impl From<FIFOCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFOCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FIFOCFG` reader - FIFO Configuration"]
pub struct FIFOCFG_R(crate::FieldReader<u8, FIFOCFG_A>);
impl FIFOCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFOCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FIFOCFG_A> {
        match self.bits {
            0 => Some(FIFOCFG_A::ALAP_CFG),
            1 => Some(FIFOCFG_A::HALF_CFG),
            2 => Some(FIFOCFG_A::ASAP_CFG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALAP_CFG`"]
    #[inline(always)]
    pub fn is_alap_cfg(&self) -> bool {
        **self == FIFOCFG_A::ALAP_CFG
    }
    #[doc = "Checks if the value of the field is `HALF_CFG`"]
    #[inline(always)]
    pub fn is_half_cfg(&self) -> bool {
        **self == FIFOCFG_A::HALF_CFG
    }
    #[doc = "Checks if the value of the field is `ASAP_CFG`"]
    #[inline(always)]
    pub fn is_asap_cfg(&self) -> bool {
        **self == FIFOCFG_A::ASAP_CFG
    }
}
impl core::ops::Deref for FIFOCFG_R {
    type Target = crate::FieldReader<u8, FIFOCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOCFG` writer - FIFO Configuration"]
pub struct FIFOCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFOCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The largest defined length AHB burst is performed on the destination AHB interface."]
    #[inline(always)]
    pub fn alap_cfg(self) -> &'a mut W {
        self.variant(FIFOCFG_A::ALAP_CFG)
    }
    #[doc = "When half FIFO size is available/filled, a source/destination request is serviced."]
    #[inline(always)]
    pub fn half_cfg(self) -> &'a mut W {
        self.variant(FIFOCFG_A::HALF_CFG)
    }
    #[doc = "When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    #[inline(always)]
    pub fn asap_cfg(self) -> &'a mut W {
        self.variant(FIFOCFG_A::ASAP_CFG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Source with Peripheral identifier"]
    #[inline(always)]
    pub fn src_per(&self) -> SRC_PER_R {
        SRC_PER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Destination with Peripheral identifier"]
    #[inline(always)]
    pub fn dst_per(&self) -> DST_PER_R {
        DST_PER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Software or Hardware Selection for the Source"]
    #[inline(always)]
    pub fn src_h2sel(&self) -> SRC_H2SEL_R {
        SRC_H2SEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software or Hardware Selection for the Destination"]
    #[inline(always)]
    pub fn dst_h2sel(&self) -> DST_H2SEL_R {
        DST_H2SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stop On Done"]
    #[inline(always)]
    pub fn sod(&self) -> SOD_R {
        SOD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interface Lock"]
    #[inline(always)]
    pub fn lock_if(&self) -> LOCK_IF_R {
        LOCK_IF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bus Lock"]
    #[inline(always)]
    pub fn lock_b(&self) -> LOCK_B_R {
        LOCK_B_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Master Interface Arbiter Lock"]
    #[inline(always)]
    pub fn lock_if_l(&self) -> LOCK_IF_L_R {
        LOCK_IF_L_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - AHB Protection"]
    #[inline(always)]
    pub fn ahb_prot(&self) -> AHB_PROT_R {
        AHB_PROT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:29 - FIFO Configuration"]
    #[inline(always)]
    pub fn fifocfg(&self) -> FIFOCFG_R {
        FIFOCFG_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source with Peripheral identifier"]
    #[inline(always)]
    pub fn src_per(&mut self) -> SRC_PER_W {
        SRC_PER_W { w: self }
    }
    #[doc = "Bits 4:7 - Destination with Peripheral identifier"]
    #[inline(always)]
    pub fn dst_per(&mut self) -> DST_PER_W {
        DST_PER_W { w: self }
    }
    #[doc = "Bit 9 - Software or Hardware Selection for the Source"]
    #[inline(always)]
    pub fn src_h2sel(&mut self) -> SRC_H2SEL_W {
        SRC_H2SEL_W { w: self }
    }
    #[doc = "Bit 13 - Software or Hardware Selection for the Destination"]
    #[inline(always)]
    pub fn dst_h2sel(&mut self) -> DST_H2SEL_W {
        DST_H2SEL_W { w: self }
    }
    #[doc = "Bit 16 - Stop On Done"]
    #[inline(always)]
    pub fn sod(&mut self) -> SOD_W {
        SOD_W { w: self }
    }
    #[doc = "Bit 20 - Interface Lock"]
    #[inline(always)]
    pub fn lock_if(&mut self) -> LOCK_IF_W {
        LOCK_IF_W { w: self }
    }
    #[doc = "Bit 21 - Bus Lock"]
    #[inline(always)]
    pub fn lock_b(&mut self) -> LOCK_B_W {
        LOCK_B_W { w: self }
    }
    #[doc = "Bit 22 - Master Interface Arbiter Lock"]
    #[inline(always)]
    pub fn lock_if_l(&mut self) -> LOCK_IF_L_W {
        LOCK_IF_L_W { w: self }
    }
    #[doc = "Bits 24:26 - AHB Protection"]
    #[inline(always)]
    pub fn ahb_prot(&mut self) -> AHB_PROT_W {
        AHB_PROT_W { w: self }
    }
    #[doc = "Bits 28:29 - FIFO Configuration"]
    #[inline(always)]
    pub fn fifocfg(&mut self) -> FIFOCFG_W {
        FIFOCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Configuration Register (ch_num = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg4](index.html) module"]
pub struct CFG4_SPEC;
impl crate::RegisterSpec for CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg4::R](R) reader structure"]
impl crate::Readable for CFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg4::W](W) writer structure"]
impl crate::Writable for CFG4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG4 to value 0x0100_0000"]
impl crate::Resettable for CFG4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
