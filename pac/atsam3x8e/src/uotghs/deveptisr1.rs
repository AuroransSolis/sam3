#[doc = "Register `DEVEPTISR1` reader"]
pub type R = crate::R<Deveptisr1Spec>;
#[doc = "Field `TXINI` reader - Transmitted IN Data Interrupt"]
pub type TxiniR = crate::BitReader;
#[doc = "Field `RXOUTI` reader - Received OUT Data Interrupt"]
pub type RxoutiR = crate::BitReader;
#[doc = "Field `RXSTPI` reader - Received SETUP Interrupt"]
pub type RxstpiR = crate::BitReader;
#[doc = "Field `NAKOUTI` reader - NAKed OUT Interrupt"]
pub type NakoutiR = crate::BitReader;
#[doc = "Field `NAKINI` reader - NAKed IN Interrupt"]
pub type NakiniR = crate::BitReader;
#[doc = "Field `OVERFI` reader - Overflow Interrupt"]
pub type OverfiR = crate::BitReader;
#[doc = "Field `STALLEDI` reader - STALLed Interrupt"]
pub type StallediR = crate::BitReader;
#[doc = "Field `SHORTPACKET` reader - Short Packet Interrupt"]
pub type ShortpacketR = crate::BitReader;
#[doc = "Data Toggle Sequence"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtseq {
    #[doc = "0: Data0 toggle sequence"]
    Data0 = 0,
    #[doc = "1: Data1 toggle sequence"]
    Data1 = 1,
    #[doc = "2: Reserved for high-bandwidth isochronous endpoint"]
    Data2 = 2,
    #[doc = "3: Reserved for high-bandwidth isochronous endpoint"]
    Mdata = 3,
}
impl From<Dtseq> for u8 {
    #[inline(always)]
    fn from(variant: Dtseq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtseq {
    type Ux = u8;
}
impl crate::IsEnum for Dtseq {}
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub type DtseqR = crate::FieldReader<Dtseq>;
impl DtseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtseq {
        match self.bits {
            0 => Dtseq::Data0,
            1 => Dtseq::Data1,
            2 => Dtseq::Data2,
            3 => Dtseq::Mdata,
            _ => unreachable!(),
        }
    }
    #[doc = "Data0 toggle sequence"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == Dtseq::Data0
    }
    #[doc = "Data1 toggle sequence"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == Dtseq::Data1
    }
    #[doc = "Reserved for high-bandwidth isochronous endpoint"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == Dtseq::Data2
    }
    #[doc = "Reserved for high-bandwidth isochronous endpoint"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == Dtseq::Mdata
    }
}
#[doc = "Number of Busy Banks"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nbusybk {
    #[doc = "0: 0 busy bank (all banks free)"]
    _0Busy = 0,
    #[doc = "1: 1 busy bank"]
    _1Busy = 1,
    #[doc = "2: 2 busy banks"]
    _2Busy = 2,
    #[doc = "3: 3 busy banks"]
    _3Busy = 3,
}
impl From<Nbusybk> for u8 {
    #[inline(always)]
    fn from(variant: Nbusybk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nbusybk {
    type Ux = u8;
}
impl crate::IsEnum for Nbusybk {}
#[doc = "Field `NBUSYBK` reader - Number of Busy Banks"]
pub type NbusybkR = crate::FieldReader<Nbusybk>;
impl NbusybkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nbusybk {
        match self.bits {
            0 => Nbusybk::_0Busy,
            1 => Nbusybk::_1Busy,
            2 => Nbusybk::_2Busy,
            3 => Nbusybk::_3Busy,
            _ => unreachable!(),
        }
    }
    #[doc = "0 busy bank (all banks free)"]
    #[inline(always)]
    pub fn is_0_busy(&self) -> bool {
        *self == Nbusybk::_0Busy
    }
    #[doc = "1 busy bank"]
    #[inline(always)]
    pub fn is_1_busy(&self) -> bool {
        *self == Nbusybk::_1Busy
    }
    #[doc = "2 busy banks"]
    #[inline(always)]
    pub fn is_2_busy(&self) -> bool {
        *self == Nbusybk::_2Busy
    }
    #[doc = "3 busy banks"]
    #[inline(always)]
    pub fn is_3_busy(&self) -> bool {
        *self == Nbusybk::_3Busy
    }
}
#[doc = "Current Bank"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Currbk {
    #[doc = "0: Current bank is bank0"]
    Bank0 = 0,
    #[doc = "1: Current bank is bank1"]
    Bank1 = 1,
    #[doc = "2: Current bank is bank2"]
    Bank2 = 2,
}
impl From<Currbk> for u8 {
    #[inline(always)]
    fn from(variant: Currbk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Currbk {
    type Ux = u8;
}
impl crate::IsEnum for Currbk {}
#[doc = "Field `CURRBK` reader - Current Bank"]
pub type CurrbkR = crate::FieldReader<Currbk>;
impl CurrbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Currbk> {
        match self.bits {
            0 => Some(Currbk::Bank0),
            1 => Some(Currbk::Bank1),
            2 => Some(Currbk::Bank2),
            _ => None,
        }
    }
    #[doc = "Current bank is bank0"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == Currbk::Bank0
    }
    #[doc = "Current bank is bank1"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == Currbk::Bank1
    }
    #[doc = "Current bank is bank2"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == Currbk::Bank2
    }
}
#[doc = "Field `RWALL` reader - Read-write Allowed"]
pub type RwallR = crate::BitReader;
#[doc = "Field `CTRLDIR` reader - Control Direction"]
pub type CtrldirR = crate::BitReader;
#[doc = "Field `CFGOK` reader - Configuration OK Status"]
pub type CfgokR = crate::BitReader;
#[doc = "Field `BYCT` reader - Byte Count"]
pub type ByctR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txini(&self) -> TxiniR {
        TxiniR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxouti(&self) -> RxoutiR {
        RxoutiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline(always)]
    pub fn rxstpi(&self) -> RxstpiR {
        RxstpiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline(always)]
    pub fn nakouti(&self) -> NakoutiR {
        NakoutiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline(always)]
    pub fn nakini(&self) -> NakiniR {
        NakiniR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfi(&self) -> OverfiR {
        OverfiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline(always)]
    pub fn stalledi(&self) -> StallediR {
        StallediR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpacket(&self) -> ShortpacketR {
        ShortpacketR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DtseqR {
        DtseqR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Number of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NbusybkR {
        NbusybkR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CurrbkR {
        CurrbkR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Read-write Allowed"]
    #[inline(always)]
    pub fn rwall(&self) -> RwallR {
        RwallR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Control Direction"]
    #[inline(always)]
    pub fn ctrldir(&self) -> CtrldirR {
        CtrldirR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration OK Status"]
    #[inline(always)]
    pub fn cfgok(&self) -> CfgokR {
        CfgokR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:30 - Byte Count"]
    #[inline(always)]
    pub fn byct(&self) -> ByctR {
        ByctR::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
#[doc = "Device Endpoint Status Register (n = 0) 1\n\nYou can [`read`](crate::Reg::read) this register and get [`deveptisr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Deveptisr1Spec;
impl crate::RegisterSpec for Deveptisr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deveptisr1::R`](R) reader structure"]
impl crate::Readable for Deveptisr1Spec {}
