#[doc = "Register `DEVEPTISR5` reader"]
pub type R = crate::R<DEVEPTISR5_SPEC>;
#[doc = "Field `TXINI` reader - Transmitted IN Data Interrupt"]
pub type TXINI_R = crate::BitReader;
#[doc = "Field `RXOUTI` reader - Received OUT Data Interrupt"]
pub type RXOUTI_R = crate::BitReader;
#[doc = "Field `RXSTPI` reader - Received SETUP Interrupt"]
pub type RXSTPI_R = crate::BitReader;
#[doc = "Field `NAKOUTI` reader - NAKed OUT Interrupt"]
pub type NAKOUTI_R = crate::BitReader;
#[doc = "Field `NAKINI` reader - NAKed IN Interrupt"]
pub type NAKINI_R = crate::BitReader;
#[doc = "Field `OVERFI` reader - Overflow Interrupt"]
pub type OVERFI_R = crate::BitReader;
#[doc = "Field `STALLEDI` reader - STALLed Interrupt"]
pub type STALLEDI_R = crate::BitReader;
#[doc = "Field `SHORTPACKET` reader - Short Packet Interrupt"]
pub type SHORTPACKET_R = crate::BitReader;
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
    #[doc = "2: Reserved for high-bandwidth isochronous endpoint"]
    Data2 = 2,
    #[doc = "3: Reserved for high-bandwidth isochronous endpoint"]
    Mdata = 3,
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
    pub fn variant(&self) -> DTSEQ_A {
        match self.bits {
            0 => DTSEQ_A::Data0,
            1 => DTSEQ_A::Data1,
            2 => DTSEQ_A::Data2,
            3 => DTSEQ_A::Mdata,
            _ => unreachable!(),
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
    #[doc = "Reserved for high-bandwidth isochronous endpoint"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == DTSEQ_A::Data2
    }
    #[doc = "Reserved for high-bandwidth isochronous endpoint"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == DTSEQ_A::Mdata
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
    pub fn variant(&self) -> NBUSYBK_A {
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
    pub fn variant(&self) -> Option<CURRBK_A> {
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
#[doc = "Field `CTRLDIR` reader - Control Direction"]
pub type CTRLDIR_R = crate::BitReader;
#[doc = "Field `CFGOK` reader - Configuration OK Status"]
pub type CFGOK_R = crate::BitReader;
#[doc = "Field `BYCT` reader - Byte Count"]
pub type BYCT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txini(&self) -> TXINI_R {
        TXINI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxouti(&self) -> RXOUTI_R {
        RXOUTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline(always)]
    pub fn rxstpi(&self) -> RXSTPI_R {
        RXSTPI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline(always)]
    pub fn nakouti(&self) -> NAKOUTI_R {
        NAKOUTI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline(always)]
    pub fn nakini(&self) -> NAKINI_R {
        NAKINI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfi(&self) -> OVERFI_R {
        OVERFI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline(always)]
    pub fn stalledi(&self) -> STALLEDI_R {
        STALLEDI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpacket(&self) -> SHORTPACKET_R {
        SHORTPACKET_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 17 - Control Direction"]
    #[inline(always)]
    pub fn ctrldir(&self) -> CTRLDIR_R {
        CTRLDIR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration OK Status"]
    #[inline(always)]
    pub fn cfgok(&self) -> CFGOK_R {
        CFGOK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:30 - Byte Count"]
    #[inline(always)]
    pub fn byct(&self) -> BYCT_R {
        BYCT_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
#[doc = "Device Endpoint Status Register (n = 0) 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptisr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTISR5_SPEC;
impl crate::RegisterSpec for DEVEPTISR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deveptisr5::R`](R) reader structure"]
impl crate::Readable for DEVEPTISR5_SPEC {}
