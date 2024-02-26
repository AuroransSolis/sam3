#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `IDTE` reader - ID Transition Interrupt Enable"]
pub type IdteR = crate::BitReader;
#[doc = "Field `IDTE` writer - ID Transition Interrupt Enable"]
pub type IdteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSTE` reader - VBus Transition Interrupt Enable"]
pub type VbusteR = crate::BitReader;
#[doc = "Field `VBUSTE` writer - VBus Transition Interrupt Enable"]
pub type VbusteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPE` reader - SRP Interrupt Enable"]
pub type SrpeR = crate::BitReader;
#[doc = "Field `SRPE` writer - SRP Interrupt Enable"]
pub type SrpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBERRE` reader - VBus Error Interrupt Enable"]
pub type VberreR = crate::BitReader;
#[doc = "Field `VBERRE` writer - VBus Error Interrupt Enable"]
pub type VberreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCERRE` reader - B-Connection Error Interrupt Enable"]
pub type BcerreR = crate::BitReader;
#[doc = "Field `BCERRE` writer - B-Connection Error Interrupt Enable"]
pub type BcerreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROLEEXE` reader - Role Exchange Interrupt Enable"]
pub type RoleexeR = crate::BitReader;
#[doc = "Field `ROLEEXE` writer - Role Exchange Interrupt Enable"]
pub type RoleexeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPERRE` reader - HNP Error Interrupt Enable"]
pub type HnperreR = crate::BitReader;
#[doc = "Field `HNPERRE` writer - HNP Error Interrupt Enable"]
pub type HnperreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOE` reader - Suspend Time-Out Interrupt Enable"]
pub type StoeR = crate::BitReader;
#[doc = "Field `STOE` writer - Suspend Time-Out Interrupt Enable"]
pub type StoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSHWC` reader - VBus Hardware Control"]
pub type VbushwcR = crate::BitReader;
#[doc = "Field `VBUSHWC` writer - VBus Hardware Control"]
pub type VbushwcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPSEL` reader - SRP Selection"]
pub type SrpselR = crate::BitReader;
#[doc = "Field `SRPSEL` writer - SRP Selection"]
pub type SrpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPREQ` reader - SRP Request"]
pub type SrpreqR = crate::BitReader;
#[doc = "Field `SRPREQ` writer - SRP Request"]
pub type SrpreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPREQ` reader - HNP Request"]
pub type HnpreqR = crate::BitReader;
#[doc = "Field `HNPREQ` writer - HNP Request"]
pub type HnpreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGPADE` reader - OTG Pad Enable"]
pub type OtgpadeR = crate::BitReader;
#[doc = "Field `OTGPADE` writer - OTG Pad Enable"]
pub type OtgpadeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSPO` reader - VBus Polarity Off"]
pub type VbuspoR = crate::BitReader;
#[doc = "Field `VBUSPO` writer - VBus Polarity Off"]
pub type VbuspoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRZCLK` reader - Freeze USB Clock"]
pub type FrzclkR = crate::BitReader;
#[doc = "Field `FRZCLK` writer - Freeze USB Clock"]
pub type FrzclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBE` reader - UOTGHS Enable"]
pub type UsbeR = crate::BitReader;
#[doc = "Field `USBE` writer - UOTGHS Enable"]
pub type UsbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMVALUE` reader - Timer Value"]
pub type TimvalueR = crate::FieldReader;
#[doc = "Field `TIMVALUE` writer - Timer Value"]
pub type TimvalueW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMPAGE` reader - Timer Page"]
pub type TimpageR = crate::FieldReader;
#[doc = "Field `TIMPAGE` writer - Timer Page"]
pub type TimpageW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UNLOCK` reader - Timer Access Unlock"]
pub type UnlockR = crate::BitReader;
#[doc = "Field `UNLOCK` writer - Timer Access Unlock"]
pub type UnlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "UOTGID Pin Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uide {
    #[doc = "0: The USB mode (device/host) is selected from the UIMOD bit."]
    Uimod = 0,
    #[doc = "1: The USB mode (device/host) is selected from the UOTGID input pin."]
    Uotgid = 1,
}
impl From<Uide> for bool {
    #[inline(always)]
    fn from(variant: Uide) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIDE` reader - UOTGID Pin Enable"]
pub type UideR = crate::BitReader<Uide>;
impl UideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uide {
        match self.bits {
            false => Uide::Uimod,
            true => Uide::Uotgid,
        }
    }
    #[doc = "The USB mode (device/host) is selected from the UIMOD bit."]
    #[inline(always)]
    pub fn is_uimod(&self) -> bool {
        *self == Uide::Uimod
    }
    #[doc = "The USB mode (device/host) is selected from the UOTGID input pin."]
    #[inline(always)]
    pub fn is_uotgid(&self) -> bool {
        *self == Uide::Uotgid
    }
}
#[doc = "Field `UIDE` writer - UOTGID Pin Enable"]
pub type UideW<'a, REG> = crate::BitWriter<'a, REG, Uide>;
impl<'a, REG> UideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The USB mode (device/host) is selected from the UIMOD bit."]
    #[inline(always)]
    pub fn uimod(self) -> &'a mut crate::W<REG> {
        self.variant(Uide::Uimod)
    }
    #[doc = "The USB mode (device/host) is selected from the UOTGID input pin."]
    #[inline(always)]
    pub fn uotgid(self) -> &'a mut crate::W<REG> {
        self.variant(Uide::Uotgid)
    }
}
#[doc = "UOTGHS Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uimod {
    #[doc = "0: The module is in USB host mode."]
    Host = 0,
    #[doc = "1: The module is in USB device mode."]
    Device = 1,
}
impl From<Uimod> for bool {
    #[inline(always)]
    fn from(variant: Uimod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIMOD` reader - UOTGHS Mode"]
pub type UimodR = crate::BitReader<Uimod>;
impl UimodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uimod {
        match self.bits {
            false => Uimod::Host,
            true => Uimod::Device,
        }
    }
    #[doc = "The module is in USB host mode."]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == Uimod::Host
    }
    #[doc = "The module is in USB device mode."]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == Uimod::Device
    }
}
#[doc = "Field `UIMOD` writer - UOTGHS Mode"]
pub type UimodW<'a, REG> = crate::BitWriter<'a, REG, Uimod>;
impl<'a, REG> UimodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The module is in USB host mode."]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(Uimod::Host)
    }
    #[doc = "The module is in USB device mode."]
    #[inline(always)]
    pub fn device(self) -> &'a mut crate::W<REG> {
        self.variant(Uimod::Device)
    }
}
impl R {
    #[doc = "Bit 0 - ID Transition Interrupt Enable"]
    #[inline(always)]
    pub fn idte(&self) -> IdteR {
        IdteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Enable"]
    #[inline(always)]
    pub fn vbuste(&self) -> VbusteR {
        VbusteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRP Interrupt Enable"]
    #[inline(always)]
    pub fn srpe(&self) -> SrpeR {
        SrpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Enable"]
    #[inline(always)]
    pub fn vberre(&self) -> VberreR {
        VberreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn bcerre(&self) -> BcerreR {
        BcerreR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Enable"]
    #[inline(always)]
    pub fn roleexe(&self) -> RoleexeR {
        RoleexeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Enable"]
    #[inline(always)]
    pub fn hnperre(&self) -> HnperreR {
        HnperreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Enable"]
    #[inline(always)]
    pub fn stoe(&self) -> StoeR {
        StoeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VBus Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&self) -> VbushwcR {
        VbushwcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRP Selection"]
    #[inline(always)]
    pub fn srpsel(&self) -> SrpselR {
        SrpselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRP Request"]
    #[inline(always)]
    pub fn srpreq(&self) -> SrpreqR {
        SrpreqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HnpreqR {
        HnpreqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG Pad Enable"]
    #[inline(always)]
    pub fn otgpade(&self) -> OtgpadeR {
        OtgpadeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VBus Polarity Off"]
    #[inline(always)]
    pub fn vbuspo(&self) -> VbuspoR {
        VbuspoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FrzclkR {
        FrzclkR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - UOTGHS Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> UsbeR {
        UsbeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Timer Value"]
    #[inline(always)]
    pub fn timvalue(&self) -> TimvalueR {
        TimvalueR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Timer Page"]
    #[inline(always)]
    pub fn timpage(&self) -> TimpageR {
        TimpageR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Timer Access Unlock"]
    #[inline(always)]
    pub fn unlock(&self) -> UnlockR {
        UnlockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - UOTGID Pin Enable"]
    #[inline(always)]
    pub fn uide(&self) -> UideR {
        UideR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UOTGHS Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UimodR {
        UimodR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn idte(&mut self) -> IdteW<CtrlSpec> {
        IdteW::new(self, 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbuste(&mut self) -> VbusteW<CtrlSpec> {
        VbusteW::new(self, 1)
    }
    #[doc = "Bit 2 - SRP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srpe(&mut self) -> SrpeW<CtrlSpec> {
        SrpeW::new(self, 2)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vberre(&mut self) -> VberreW<CtrlSpec> {
        VberreW::new(self, 3)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcerre(&mut self) -> BcerreW<CtrlSpec> {
        BcerreW::new(self, 4)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn roleexe(&mut self) -> RoleexeW<CtrlSpec> {
        RoleexeW::new(self, 5)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hnperre(&mut self) -> HnperreW<CtrlSpec> {
        HnperreW::new(self, 6)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stoe(&mut self) -> StoeW<CtrlSpec> {
        StoeW::new(self, 7)
    }
    #[doc = "Bit 8 - VBus Hardware Control"]
    #[inline(always)]
    #[must_use]
    pub fn vbushwc(&mut self) -> VbushwcW<CtrlSpec> {
        VbushwcW::new(self, 8)
    }
    #[doc = "Bit 9 - SRP Selection"]
    #[inline(always)]
    #[must_use]
    pub fn srpsel(&mut self) -> SrpselW<CtrlSpec> {
        SrpselW::new(self, 9)
    }
    #[doc = "Bit 10 - SRP Request"]
    #[inline(always)]
    #[must_use]
    pub fn srpreq(&mut self) -> SrpreqW<CtrlSpec> {
        SrpreqW::new(self, 10)
    }
    #[doc = "Bit 11 - HNP Request"]
    #[inline(always)]
    #[must_use]
    pub fn hnpreq(&mut self) -> HnpreqW<CtrlSpec> {
        HnpreqW::new(self, 11)
    }
    #[doc = "Bit 12 - OTG Pad Enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgpade(&mut self) -> OtgpadeW<CtrlSpec> {
        OtgpadeW::new(self, 12)
    }
    #[doc = "Bit 13 - VBus Polarity Off"]
    #[inline(always)]
    #[must_use]
    pub fn vbuspo(&mut self) -> VbuspoW<CtrlSpec> {
        VbuspoW::new(self, 13)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    #[must_use]
    pub fn frzclk(&mut self) -> FrzclkW<CtrlSpec> {
        FrzclkW::new(self, 14)
    }
    #[doc = "Bit 15 - UOTGHS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbe(&mut self) -> UsbeW<CtrlSpec> {
        UsbeW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Timer Value"]
    #[inline(always)]
    #[must_use]
    pub fn timvalue(&mut self) -> TimvalueW<CtrlSpec> {
        TimvalueW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Timer Page"]
    #[inline(always)]
    #[must_use]
    pub fn timpage(&mut self) -> TimpageW<CtrlSpec> {
        TimpageW::new(self, 20)
    }
    #[doc = "Bit 22 - Timer Access Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UnlockW<CtrlSpec> {
        UnlockW::new(self, 22)
    }
    #[doc = "Bit 24 - UOTGID Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uide(&mut self) -> UideW<CtrlSpec> {
        UideW::new(self, 24)
    }
    #[doc = "Bit 25 - UOTGHS Mode"]
    #[inline(always)]
    #[must_use]
    pub fn uimod(&mut self) -> UimodW<CtrlSpec> {
        UimodW::new(self, 25)
    }
}
#[doc = "General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0300_4000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0300_4000;
}
