#[doc = "Register `DEVEPTICR3` writer"]
pub type W = crate::W<DEVEPTICR3_SPEC>;
#[doc = "Field `TXINIC` writer - Transmitted IN Data Interrupt Clear"]
pub type TXINIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIC` writer - Received OUT Data Interrupt Clear"]
pub type RXOUTIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTPIC` writer - Received SETUP Interrupt Clear"]
pub type RXSTPIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKOUTIC` writer - NAKed OUT Interrupt Clear"]
pub type NAKOUTIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINIC` writer - NAKed IN Interrupt Clear"]
pub type NAKINIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OVERFIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLEDIC` writer - STALLed Interrupt Clear"]
pub type STALLEDIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETC` writer - Short Packet Interrupt Clear"]
pub type SHORTPACKETC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinic(&mut self) -> TXINIC_W<DEVEPTICR3_SPEC> {
        TXINIC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutic(&mut self) -> RXOUTIC_W<DEVEPTICR3_SPEC> {
        RXOUTIC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxstpic(&mut self) -> RXSTPIC_W<DEVEPTICR3_SPEC> {
        RXSTPIC_W::new(self, 2)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakoutic(&mut self) -> NAKOUTIC_W<DEVEPTICR3_SPEC> {
        NAKOUTIC_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn nakinic(&mut self) -> NAKINIC_W<DEVEPTICR3_SPEC> {
        NAKINIC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OVERFIC_W<DEVEPTICR3_SPEC> {
        OVERFIC_W::new(self, 5)
    }
    #[doc = "Bit 6 - STALLed Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn stalledic(&mut self) -> STALLEDIC_W<DEVEPTICR3_SPEC> {
        STALLEDIC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketc(&mut self) -> SHORTPACKETC_W<DEVEPTICR3_SPEC> {
        SHORTPACKETC_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Endpoint Clear Register (n = 0) 3\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devepticr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTICR3_SPEC;
impl crate::RegisterSpec for DEVEPTICR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devepticr3::W`](W) writer structure"]
impl crate::Writable for DEVEPTICR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
