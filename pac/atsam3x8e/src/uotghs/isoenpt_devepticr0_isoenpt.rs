#[doc = "Register `DEVEPTICR0_ISOENPT` writer"]
pub type W = crate::W<ISOENPT_DEVEPTICR0_ISOENPT_SPEC>;
#[doc = "Field `TXINIC` writer - Transmitted IN Data Interrupt Clear"]
pub type TXINIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUTIC` writer - Received OUT Data Interrupt Clear"]
pub type RXOUTIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFIC` writer - Underflow Interrupt Clear"]
pub type UNDERFIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOINERRIC` writer - High bandwidth isochronous IN Underflow Error Interrupt Clear"]
pub type HBISOINERRIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBISOFLUSHIC` writer - High Bandwidth Isochronous IN Flush Interrupt Clear"]
pub type HBISOFLUSHIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OVERFIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERRIC` writer - CRC Error Interrupt Clear"]
pub type CRCERRIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTPACKETC` writer - Short Packet Interrupt Clear"]
pub type SHORTPACKETC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn txinic(&mut self) -> TXINIC_W<ISOENPT_DEVEPTICR0_ISOENPT_SPEC> {
        TXINIC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxoutic(&mut self) -> RXOUTIC_W<ISOENPT_DEVEPTICR0_ISOENPT_SPEC> {
        RXOUTIC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn underfic(&mut self) -> UNDERFIC_W<ISOENPT_DEVEPTICR0_ISOENPT_SPEC> {
        UNDERFIC_W::new(self, 2)
    }
    #[doc = "Bit 3 - High bandwidth isochronous IN Underflow Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoinerric(&mut self) -> HBISOINERRIC_W<ISOENPT_DEVEPTICR0_ISOENPT_SPEC> {
        HBISOINERRIC_W::new(self, 3)
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hbisoflushic(&mut self) -> HBISOFLUSHIC_W<ISOENPT_DEVEPTICR0_ISOENPT_SPEC> {
        HBISOFLUSHIC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn overfic(&mut self) -> OVERFIC_W<ISOENPT_DEVEPTICR0_ISOENPT_SPEC> {
        OVERFIC_W::new(self, 5)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn crcerric(&mut self) -> CRCERRIC_W<ISOENPT_DEVEPTICR0_ISOENPT_SPEC> {
        CRCERRIC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn shortpacketc(&mut self) -> SHORTPACKETC_W<ISOENPT_DEVEPTICR0_ISOENPT_SPEC> {
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
#[doc = "Device Endpoint Clear Register (n = 0)\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isoenpt_devepticr0_isoenpt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISOENPT_DEVEPTICR0_ISOENPT_SPEC;
impl crate::RegisterSpec for ISOENPT_DEVEPTICR0_ISOENPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isoenpt_devepticr0_isoenpt::W`](W) writer structure"]
impl crate::Writable for ISOENPT_DEVEPTICR0_ISOENPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
