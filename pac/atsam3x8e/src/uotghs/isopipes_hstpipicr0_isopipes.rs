#[doc = "Register `HSTPIPICR0_ISOPIPES` writer"]
pub struct W(crate::W<ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC>;
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
impl From<crate::W<ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINIC` writer - Received IN Data Interrupt Clear"]
pub type RXINIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC, bool, O>;
#[doc = "Field `TXOUTIC` writer - Transmitted OUT Data Interrupt Clear"]
pub type TXOUTIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC, bool, O>;
#[doc = "Field `UNDERFIC` writer - Underflow Interrupt Clear"]
pub type UNDERFIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC, bool, O>;
#[doc = "Field `NAKEDIC` writer - NAKed Interrupt Clear"]
pub type NAKEDIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC, bool, O>;
#[doc = "Field `OVERFIC` writer - Overflow Interrupt Clear"]
pub type OVERFIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC, bool, O>;
#[doc = "Field `CRCERRIC` writer - CRC Error Interrupt Clear"]
pub type CRCERRIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC, bool, O>;
#[doc = "Field `SHORTPACKETIC` writer - Short Packet Interrupt Clear"]
pub type SHORTPACKETIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Clear"]
    #[inline(always)]
    pub fn rxinic(&mut self) -> RXINIC_W<0> {
        RXINIC_W::new(self)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Clear"]
    #[inline(always)]
    pub fn txoutic(&mut self) -> TXOUTIC_W<1> {
        TXOUTIC_W::new(self)
    }
    #[doc = "Bit 2 - Underflow Interrupt Clear"]
    #[inline(always)]
    pub fn underfic(&mut self) -> UNDERFIC_W<2> {
        UNDERFIC_W::new(self)
    }
    #[doc = "Bit 4 - NAKed Interrupt Clear"]
    #[inline(always)]
    pub fn nakedic(&mut self) -> NAKEDIC_W<4> {
        NAKEDIC_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Clear"]
    #[inline(always)]
    pub fn overfic(&mut self) -> OVERFIC_W<5> {
        OVERFIC_W::new(self)
    }
    #[doc = "Bit 6 - CRC Error Interrupt Clear"]
    #[inline(always)]
    pub fn crcerric(&mut self) -> CRCERRIC_W<6> {
        CRCERRIC_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Clear"]
    #[inline(always)]
    pub fn shortpacketic(&mut self) -> SHORTPACKETIC_W<7> {
        SHORTPACKETIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Clear Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isopipes_hstpipicr0_isopipes](index.html) module"]
pub struct ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC;
impl crate::RegisterSpec for ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [isopipes_hstpipicr0_isopipes::W](W) writer structure"]
impl crate::Writable for ISOPIPES_HSTPIPICR0_ISOPIPES_SPEC {
    type Writer = W;
}
