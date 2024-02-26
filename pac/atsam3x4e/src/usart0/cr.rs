#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RstrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RsttxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub type RststaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STTBRK` writer - Start Break"]
pub type SttbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPBRK` writer - Stop Break"]
pub type StpbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STTTO` writer - Start Time-out"]
pub type StttoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENDA` writer - Send Address"]
pub type SendaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIT` writer - Reset Iterations"]
pub type RstitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTNACK` writer - Reset Non Acknowledge"]
pub type RstnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETTO` writer - Rearm Time-out"]
pub type RettoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` writer - Request to Send Enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSDIS` writer - Request to Send Disable"]
pub type RtsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINABT` writer - Abort LIN Transmission"]
pub type LinabtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINWKUP` writer - Send LIN Wakeup Signal"]
pub type LinwkupW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rstrx(&mut self) -> RstrxW<CrSpec> {
        RstrxW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn rsttx(&mut self) -> RsttxW<CrSpec> {
        RsttxW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RxenW<CrSpec> {
        RxenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RxdisW<CrSpec> {
        RxdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TxenW<CrSpec> {
        TxenW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TxdisW<CrSpec> {
        TxdisW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rststa(&mut self) -> RststaW<CrSpec> {
        RststaW::new(self, 8)
    }
    #[doc = "Bit 9 - Start Break"]
    #[inline(always)]
    #[must_use]
    pub fn sttbrk(&mut self) -> SttbrkW<CrSpec> {
        SttbrkW::new(self, 9)
    }
    #[doc = "Bit 10 - Stop Break"]
    #[inline(always)]
    #[must_use]
    pub fn stpbrk(&mut self) -> StpbrkW<CrSpec> {
        StpbrkW::new(self, 10)
    }
    #[doc = "Bit 11 - Start Time-out"]
    #[inline(always)]
    #[must_use]
    pub fn sttto(&mut self) -> StttoW<CrSpec> {
        StttoW::new(self, 11)
    }
    #[doc = "Bit 12 - Send Address"]
    #[inline(always)]
    #[must_use]
    pub fn senda(&mut self) -> SendaW<CrSpec> {
        SendaW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset Iterations"]
    #[inline(always)]
    #[must_use]
    pub fn rstit(&mut self) -> RstitW<CrSpec> {
        RstitW::new(self, 13)
    }
    #[doc = "Bit 14 - Reset Non Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn rstnack(&mut self) -> RstnackW<CrSpec> {
        RstnackW::new(self, 14)
    }
    #[doc = "Bit 15 - Rearm Time-out"]
    #[inline(always)]
    #[must_use]
    pub fn retto(&mut self) -> RettoW<CrSpec> {
        RettoW::new(self, 15)
    }
    #[doc = "Bit 18 - Request to Send Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RtsenW<CrSpec> {
        RtsenW::new(self, 18)
    }
    #[doc = "Bit 19 - Request to Send Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsdis(&mut self) -> RtsdisW<CrSpec> {
        RtsdisW::new(self, 19)
    }
    #[doc = "Bit 20 - Abort LIN Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn linabt(&mut self) -> LinabtW<CrSpec> {
        LinabtW::new(self, 20)
    }
    #[doc = "Bit 21 - Send LIN Wakeup Signal"]
    #[inline(always)]
    #[must_use]
    pub fn linwkup(&mut self) -> LinwkupW<CrSpec> {
        LinwkupW::new(self, 21)
    }
}
#[doc = "Control Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
