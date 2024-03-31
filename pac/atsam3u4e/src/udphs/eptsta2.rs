#[doc = "Register `EPTSTA2` reader"]
pub type R = crate::R<Eptsta2Spec>;
#[doc = "Field `FRCESTALL` reader - Stall Handshake Request"]
pub type FrcestallR = crate::BitReader;
#[doc = "Toggle Sequencing\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TogglesqSta {
    #[doc = "0: DATA0"]
    Data0 = 0,
    #[doc = "1: DATA1"]
    Data1 = 1,
    #[doc = "2: Reserved for High Bandwidth Isochronous Endpoint"]
    Data2 = 2,
    #[doc = "3: Reserved for High Bandwidth Isochronous Endpoint"]
    Mdata = 3,
}
impl From<TogglesqSta> for u8 {
    #[inline(always)]
    fn from(variant: TogglesqSta) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TogglesqSta {
    type Ux = u8;
}
impl crate::IsEnum for TogglesqSta {}
#[doc = "Field `TOGGLESQ_STA` reader - Toggle Sequencing"]
pub type TogglesqStaR = crate::FieldReader<TogglesqSta>;
impl TogglesqStaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TogglesqSta {
        match self.bits {
            0 => TogglesqSta::Data0,
            1 => TogglesqSta::Data1,
            2 => TogglesqSta::Data2,
            3 => TogglesqSta::Mdata,
            _ => unreachable!(),
        }
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == TogglesqSta::Data0
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == TogglesqSta::Data1
    }
    #[doc = "Reserved for High Bandwidth Isochronous Endpoint"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == TogglesqSta::Data2
    }
    #[doc = "Reserved for High Bandwidth Isochronous Endpoint"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool {
        *self == TogglesqSta::Mdata
    }
}
#[doc = "Field `ERR_OVFLW` reader - Overflow Error"]
pub type ErrOvflwR = crate::BitReader;
#[doc = "Field `RXRDY_TXKL` reader - Received OUT Data/KILL Bank"]
pub type RxrdyTxklR = crate::BitReader;
#[doc = "Field `TX_COMPLT` reader - Transmitted IN Data Complete"]
pub type TxCompltR = crate::BitReader;
#[doc = "Field `TXRDY` reader - TX Packet Ready"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `RX_SETUP` reader - Received SETUP"]
pub type RxSetupR = crate::BitReader;
#[doc = "Field `STALL_SNT` reader - Stall Sent"]
pub type StallSntR = crate::BitReader;
#[doc = "Field `NAK_IN` reader - NAK IN"]
pub type NakInR = crate::BitReader;
#[doc = "Field `NAK_OUT` reader - NAK OUT"]
pub type NakOutR = crate::BitReader;
#[doc = "Field `CURBK_CTLDIR` reader - Current Bank/Control Direction"]
pub type CurbkCtldirR = crate::FieldReader;
#[doc = "Busy Bank Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusyBankSta {
    #[doc = "0: 1 busy bank"]
    _1busybank = 0,
    #[doc = "1: 2 busy banks"]
    _2busybanks = 1,
    #[doc = "2: 3 busy banks"]
    _3busybanks = 2,
}
impl From<BusyBankSta> for u8 {
    #[inline(always)]
    fn from(variant: BusyBankSta) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BusyBankSta {
    type Ux = u8;
}
impl crate::IsEnum for BusyBankSta {}
#[doc = "Field `BUSY_BANK_STA` reader - Busy Bank Number"]
pub type BusyBankStaR = crate::FieldReader<BusyBankSta>;
impl BusyBankStaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BusyBankSta> {
        match self.bits {
            0 => Some(BusyBankSta::_1busybank),
            1 => Some(BusyBankSta::_2busybanks),
            2 => Some(BusyBankSta::_3busybanks),
            _ => None,
        }
    }
    #[doc = "1 busy bank"]
    #[inline(always)]
    pub fn is_1busybank(&self) -> bool {
        *self == BusyBankSta::_1busybank
    }
    #[doc = "2 busy banks"]
    #[inline(always)]
    pub fn is_2busybanks(&self) -> bool {
        *self == BusyBankSta::_2busybanks
    }
    #[doc = "3 busy banks"]
    #[inline(always)]
    pub fn is_3busybanks(&self) -> bool {
        *self == BusyBankSta::_3busybanks
    }
}
#[doc = "Field `BYTE_COUNT` reader - UDPHS Byte Count"]
pub type ByteCountR = crate::FieldReader<u16>;
#[doc = "Field `SHRT_PCKT` reader - Short Packet"]
pub type ShrtPcktR = crate::BitReader;
impl R {
    #[doc = "Bit 5 - Stall Handshake Request"]
    #[inline(always)]
    pub fn frcestall(&self) -> FrcestallR {
        FrcestallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Toggle Sequencing"]
    #[inline(always)]
    pub fn togglesq_sta(&self) -> TogglesqStaR {
        TogglesqStaR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Overflow Error"]
    #[inline(always)]
    pub fn err_ovflw(&self) -> ErrOvflwR {
        ErrOvflwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Received OUT Data/KILL Bank"]
    #[inline(always)]
    pub fn rxrdy_txkl(&self) -> RxrdyTxklR {
        RxrdyTxklR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmitted IN Data Complete"]
    #[inline(always)]
    pub fn tx_complt(&self) -> TxCompltR {
        TxCompltR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TX Packet Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Received SETUP"]
    #[inline(always)]
    pub fn rx_setup(&self) -> RxSetupR {
        RxSetupR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Stall Sent"]
    #[inline(always)]
    pub fn stall_snt(&self) -> StallSntR {
        StallSntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NAK IN"]
    #[inline(always)]
    pub fn nak_in(&self) -> NakInR {
        NakInR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NAK OUT"]
    #[inline(always)]
    pub fn nak_out(&self) -> NakOutR {
        NakOutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Current Bank/Control Direction"]
    #[inline(always)]
    pub fn curbk_ctldir(&self) -> CurbkCtldirR {
        CurbkCtldirR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Busy Bank Number"]
    #[inline(always)]
    pub fn busy_bank_sta(&self) -> BusyBankStaR {
        BusyBankStaR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:30 - UDPHS Byte Count"]
    #[inline(always)]
    pub fn byte_count(&self) -> ByteCountR {
        ByteCountR::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - Short Packet"]
    #[inline(always)]
    pub fn shrt_pckt(&self) -> ShrtPcktR {
        ShrtPcktR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "UDPHS Endpoint Status Register (endpoint = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eptsta2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eptsta2Spec;
impl crate::RegisterSpec for Eptsta2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eptsta2::R`](R) reader structure"]
impl crate::Readable for Eptsta2Spec {}
#[doc = "`reset()` method sets EPTSTA2 to value 0x40"]
impl crate::Resettable for Eptsta2Spec {
    const RESET_VALUE: u32 = 0x40;
}
