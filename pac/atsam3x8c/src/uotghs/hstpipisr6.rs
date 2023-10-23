#[doc = "Register `HSTPIPISR6` reader"]
pub type R = crate::R<HSTPIPISR6_SPEC>;
#[doc = "Field `RXINI` reader - Received IN Data Interrupt"]
pub type RXINI_R = crate::BitReader;
#[doc = "Field `TXOUTI` reader - Transmitted OUT Data Interrupt"]
pub type TXOUTI_R = crate::BitReader;
#[doc = "Field `TXSTPI` reader - Transmitted SETUP Interrupt"]
pub type TXSTPI_R = crate::BitReader;
#[doc = "Field `PERRI` reader - Pipe Error Interrupt"]
pub type PERRI_R = crate::BitReader;
#[doc = "Field `NAKEDI` reader - NAKed Interrupt"]
pub type NAKEDI_R = crate::BitReader;
#[doc = "Field `OVERFI` reader - Overflow Interrupt"]
pub type OVERFI_R = crate::BitReader;
#[doc = "Field `RXSTALLDI` reader - Received STALLed Interrupt"]
pub type RXSTALLDI_R = crate::BitReader;
#[doc = "Field `SHORTPACKETI` reader - Short Packet Interrupt"]
pub type SHORTPACKETI_R = crate::BitReader;
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub type DTSEQ_R = crate::FieldReader<DTSEQ_A>;
#[doc = "Data Toggle Sequence"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTSEQ_A {
    #[doc = "0: Data0 toggle sequence"]
    Data0 = 0,
    #[doc = "1: Data1 toggle sequence"]
    Data1 = 1,
}
impl From<DTSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: DTSEQ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTSEQ_A {
    type Ux = u8;
}
impl DTSEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DTSEQ_A> {
        match self.bits {
            0 => Some(DTSEQ_A::Data0),
            1 => Some(DTSEQ_A::Data1),
            _ => None,
        }
    }
    #[doc = "Data0 toggle sequence"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DTSEQ_A::Data0
    }
    #[doc = "Data1 toggle sequence"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DTSEQ_A::Data1
    }
}
#[doc = "Field `NBUSYBK` reader - Number of Busy Banks"]
pub type NBUSYBK_R = crate::FieldReader<NBUSYBK_A>;
#[doc = "Number of Busy Banks"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NBUSYBK_A {
    #[doc = "0: 0 busy bank (all banks free)"]
    _0Busy = 0,
    #[doc = "1: 1 busy bank"]
    _1Busy = 1,
    #[doc = "2: 2 busy banks"]
    _2Busy = 2,
    #[doc = "3: 3 busy banks"]
    _3Busy = 3,
}
impl From<NBUSYBK_A> for u8 {
    #[inline(always)]
    fn from(variant: NBUSYBK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NBUSYBK_A {
    type Ux = u8;
}
impl NBUSYBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NBUSYBK_A {
        match self.bits {
            0 => NBUSYBK_A::_0Busy,
            1 => NBUSYBK_A::_1Busy,
            2 => NBUSYBK_A::_2Busy,
            3 => NBUSYBK_A::_3Busy,
            _ => unreachable!(),
        }
    }
    #[doc = "0 busy bank (all banks free)"]
    #[inline(always)]
    pub fn is_0_busy(&self) -> bool {
        *self == NBUSYBK_A::_0Busy
    }
    #[doc = "1 busy bank"]
    #[inline(always)]
    pub fn is_1_busy(&self) -> bool {
        *self == NBUSYBK_A::_1Busy
    }
    #[doc = "2 busy banks"]
    #[inline(always)]
    pub fn is_2_busy(&self) -> bool {
        *self == NBUSYBK_A::_2Busy
    }
    #[doc = "3 busy banks"]
    #[inline(always)]
    pub fn is_3_busy(&self) -> bool {
        *self == NBUSYBK_A::_3Busy
    }
}
#[doc = "Field `CURRBK` reader - Current Bank"]
pub type CURRBK_R = crate::FieldReader<CURRBK_A>;
#[doc = "Current Bank"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CURRBK_A {
    #[doc = "0: Current bank is bank0"]
    Bank0 = 0,
    #[doc = "1: Current bank is bank1"]
    Bank1 = 1,
    #[doc = "2: Current bank is bank2"]
    Bank2 = 2,
}
impl From<CURRBK_A> for u8 {
    #[inline(always)]
    fn from(variant: CURRBK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CURRBK_A {
    type Ux = u8;
}
impl CURRBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CURRBK_A> {
        match self.bits {
            0 => Some(CURRBK_A::Bank0),
            1 => Some(CURRBK_A::Bank1),
            2 => Some(CURRBK_A::Bank2),
            _ => None,
        }
    }
    #[doc = "Current bank is bank0"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == CURRBK_A::Bank0
    }
    #[doc = "Current bank is bank1"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == CURRBK_A::Bank1
    }
    #[doc = "Current bank is bank2"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == CURRBK_A::Bank2
    }
}
#[doc = "Field `RWALL` reader - Read-write Allowed"]
pub type RWALL_R = crate::BitReader;
#[doc = "Field `CFGOK` reader - Configuration OK Status"]
pub type CFGOK_R = crate::BitReader;
#[doc = "Field `PBYCT` reader - Pipe Byte Count"]
pub type PBYCT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Received IN Data Interrupt"]
    #[inline(always)]
    pub fn rxini(&self) -> RXINI_R {
        RXINI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt"]
    #[inline(always)]
    pub fn txouti(&self) -> TXOUTI_R {
        TXOUTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt"]
    #[inline(always)]
    pub fn txstpi(&self) -> TXSTPI_R {
        TXSTPI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt"]
    #[inline(always)]
    pub fn perri(&self) -> PERRI_R {
        PERRI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed Interrupt"]
    #[inline(always)]
    pub fn nakedi(&self) -> NAKEDI_R {
        NAKEDI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfi(&self) -> OVERFI_R {
        OVERFI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt"]
    #[inline(always)]
    pub fn rxstalldi(&self) -> RXSTALLDI_R {
        RXSTALLDI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpacketi(&self) -> SHORTPACKETI_R {
        SHORTPACKETI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Number of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Read-write Allowed"]
    #[inline(always)]
    pub fn rwall(&self) -> RWALL_R {
        RWALL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration OK Status"]
    #[inline(always)]
    pub fn cfgok(&self) -> CFGOK_R {
        CFGOK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:30 - Pipe Byte Count"]
    #[inline(always)]
    pub fn pbyct(&self) -> PBYCT_R {
        PBYCT_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
#[doc = "Host Pipe Status Register (n = 0) 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstpipisr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTPIPISR6_SPEC;
impl crate::RegisterSpec for HSTPIPISR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstpipisr6::R`](R) reader structure"]
impl crate::Readable for HSTPIPISR6_SPEC {}
