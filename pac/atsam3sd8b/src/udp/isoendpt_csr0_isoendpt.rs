#[doc = "Register `CSR0_ISOENDPT` reader"]
pub struct R(crate::R<ISOENDPT_CSR0_ISOENDPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOENDPT_CSR0_ISOENDPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOENDPT_CSR0_ISOENDPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOENDPT_CSR0_ISOENDPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR0_ISOENDPT` writer"]
pub struct W(crate::W<ISOENDPT_CSR0_ISOENDPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOENDPT_CSR0_ISOENDPT_SPEC>;
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
impl From<crate::W<ISOENDPT_CSR0_ISOENDPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOENDPT_CSR0_ISOENDPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCOMP` reader - Generates an IN Packet with Data Previously Written in the DPR"]
pub type TXCOMP_R = crate::BitReader<bool>;
#[doc = "Field `TXCOMP` writer - Generates an IN Packet with Data Previously Written in the DPR"]
pub type TXCOMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `RX_DATA_BK0` reader - Receive Data Bank 0"]
pub type RX_DATA_BK0_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_BK0` writer - Receive Data Bank 0"]
pub type RX_DATA_BK0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `RXSETUP` reader - Received Setup"]
pub type RXSETUP_R = crate::BitReader<bool>;
#[doc = "Field `RXSETUP` writer - Received Setup"]
pub type RXSETUP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `ISOERROR` reader - A CRC error has been detected in an isochronous transfer"]
pub type ISOERROR_R = crate::BitReader<bool>;
#[doc = "Field `ISOERROR` writer - A CRC error has been detected in an isochronous transfer"]
pub type ISOERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `TXPKTRDY` reader - Transmit Packet Ready"]
pub type TXPKTRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXPKTRDY` writer - Transmit Packet Ready"]
pub type TXPKTRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `FORCESTALL` reader - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
pub type FORCESTALL_R = crate::BitReader<bool>;
#[doc = "Field `FORCESTALL` writer - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
pub type FORCESTALL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `RX_DATA_BK1` reader - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
pub type RX_DATA_BK1_R = crate::BitReader<bool>;
#[doc = "Field `RX_DATA_BK1` writer - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
pub type RX_DATA_BK1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `DIR` reader - Transfer Direction (only available for control endpoints)"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Transfer Direction (only available for control endpoints)"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<u8, EPTYPE_A>;
#[doc = "Endpoint Type"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control"]
    CTRL = 0,
    #[doc = "1: Isochronous OUT"]
    ISO_OUT = 1,
    #[doc = "5: Isochronous IN"]
    ISO_IN = 5,
    #[doc = "2: Bulk OUT"]
    BULK_OUT = 2,
    #[doc = "6: Bulk IN"]
    BULK_IN = 6,
    #[doc = "3: Interrupt OUT"]
    INT_OUT = 3,
    #[doc = "7: Interrupt IN"]
    INT_IN = 7,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
impl EPTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPTYPE_A> {
        match self.bits {
            0 => Some(EPTYPE_A::CTRL),
            1 => Some(EPTYPE_A::ISO_OUT),
            5 => Some(EPTYPE_A::ISO_IN),
            2 => Some(EPTYPE_A::BULK_OUT),
            6 => Some(EPTYPE_A::BULK_IN),
            3 => Some(EPTYPE_A::INT_OUT),
            7 => Some(EPTYPE_A::INT_IN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == EPTYPE_A::CTRL
    }
    #[doc = "Checks if the value of the field is `ISO_OUT`"]
    #[inline(always)]
    pub fn is_iso_out(&self) -> bool {
        *self == EPTYPE_A::ISO_OUT
    }
    #[doc = "Checks if the value of the field is `ISO_IN`"]
    #[inline(always)]
    pub fn is_iso_in(&self) -> bool {
        *self == EPTYPE_A::ISO_IN
    }
    #[doc = "Checks if the value of the field is `BULK_OUT`"]
    #[inline(always)]
    pub fn is_bulk_out(&self) -> bool {
        *self == EPTYPE_A::BULK_OUT
    }
    #[doc = "Checks if the value of the field is `BULK_IN`"]
    #[inline(always)]
    pub fn is_bulk_in(&self) -> bool {
        *self == EPTYPE_A::BULK_IN
    }
    #[doc = "Checks if the value of the field is `INT_OUT`"]
    #[inline(always)]
    pub fn is_int_out(&self) -> bool {
        *self == EPTYPE_A::INT_OUT
    }
    #[doc = "Checks if the value of the field is `INT_IN`"]
    #[inline(always)]
    pub fn is_int_in(&self) -> bool {
        *self == EPTYPE_A::INT_IN
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EPTYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, u8, EPTYPE_A, 3, O>;
impl<'a, const O: u8> EPTYPE_W<'a, O> {
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(EPTYPE_A::CTRL)
    }
    #[doc = "Isochronous OUT"]
    #[inline(always)]
    pub fn iso_out(self) -> &'a mut W {
        self.variant(EPTYPE_A::ISO_OUT)
    }
    #[doc = "Isochronous IN"]
    #[inline(always)]
    pub fn iso_in(self) -> &'a mut W {
        self.variant(EPTYPE_A::ISO_IN)
    }
    #[doc = "Bulk OUT"]
    #[inline(always)]
    pub fn bulk_out(self) -> &'a mut W {
        self.variant(EPTYPE_A::BULK_OUT)
    }
    #[doc = "Bulk IN"]
    #[inline(always)]
    pub fn bulk_in(self) -> &'a mut W {
        self.variant(EPTYPE_A::BULK_IN)
    }
    #[doc = "Interrupt OUT"]
    #[inline(always)]
    pub fn int_out(self) -> &'a mut W {
        self.variant(EPTYPE_A::INT_OUT)
    }
    #[doc = "Interrupt IN"]
    #[inline(always)]
    pub fn int_in(self) -> &'a mut W {
        self.variant(EPTYPE_A::INT_IN)
    }
}
#[doc = "Field `DTGLE` reader - Data Toggle"]
pub type DTGLE_R = crate::BitReader<bool>;
#[doc = "Field `DTGLE` writer - Data Toggle"]
pub type DTGLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `EPEDS` reader - Endpoint Enable Disable"]
pub type EPEDS_R = crate::BitReader<bool>;
#[doc = "Field `EPEDS` writer - Endpoint Enable Disable"]
pub type EPEDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, bool, O>;
#[doc = "Field `RXBYTECNT` reader - Number of Bytes Available in the FIFO"]
pub type RXBYTECNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXBYTECNT` writer - Number of Bytes Available in the FIFO"]
pub type RXBYTECNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ISOENDPT_CSR0_ISOENDPT_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bit 0 - Generates an IN Packet with Data Previously Written in the DPR"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline(always)]
    pub fn rx_data_bk0(&self) -> RX_DATA_BK0_R {
        RX_DATA_BK0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline(always)]
    pub fn rxsetup(&self) -> RXSETUP_R {
        RXSETUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A CRC error has been detected in an isochronous transfer"]
    #[inline(always)]
    pub fn isoerror(&self) -> ISOERROR_R {
        ISOERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn txpktrdy(&self) -> TXPKTRDY_R {
        TXPKTRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline(always)]
    pub fn forcestall(&self) -> FORCESTALL_R {
        FORCESTALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline(always)]
    pub fn rx_data_bk1(&self) -> RX_DATA_BK1_R {
        RX_DATA_BK1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints)"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Data Toggle"]
    #[inline(always)]
    pub fn dtgle(&self) -> DTGLE_R {
        DTGLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline(always)]
    pub fn epeds(&self) -> EPEDS_R {
        EPEDS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO"]
    #[inline(always)]
    pub fn rxbytecnt(&self) -> RXBYTECNT_R {
        RXBYTECNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Generates an IN Packet with Data Previously Written in the DPR"]
    #[inline(always)]
    pub fn txcomp(&mut self) -> TXCOMP_W<0> {
        TXCOMP_W::new(self)
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline(always)]
    pub fn rx_data_bk0(&mut self) -> RX_DATA_BK0_W<1> {
        RX_DATA_BK0_W::new(self)
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline(always)]
    pub fn rxsetup(&mut self) -> RXSETUP_W<2> {
        RXSETUP_W::new(self)
    }
    #[doc = "Bit 3 - A CRC error has been detected in an isochronous transfer"]
    #[inline(always)]
    pub fn isoerror(&mut self) -> ISOERROR_W<3> {
        ISOERROR_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn txpktrdy(&mut self) -> TXPKTRDY_W<4> {
        TXPKTRDY_W::new(self)
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline(always)]
    pub fn forcestall(&mut self) -> FORCESTALL_W<5> {
        FORCESTALL_W::new(self)
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline(always)]
    pub fn rx_data_bk1(&mut self) -> RX_DATA_BK1_W<6> {
        RX_DATA_BK1_W::new(self)
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints)"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<7> {
        DIR_W::new(self)
    }
    #[doc = "Bits 8:10 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W<8> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bit 11 - Data Toggle"]
    #[inline(always)]
    pub fn dtgle(&mut self) -> DTGLE_W<11> {
        DTGLE_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline(always)]
    pub fn epeds(&mut self) -> EPEDS_W<15> {
        EPEDS_W::new(self)
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO"]
    #[inline(always)]
    pub fn rxbytecnt(&mut self) -> RXBYTECNT_W<16> {
        RXBYTECNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoendpt_csr0_isoendpt](index.html) module"]
pub struct ISOENDPT_CSR0_ISOENDPT_SPEC;
impl crate::RegisterSpec for ISOENDPT_CSR0_ISOENDPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isoendpt_csr0_isoendpt::R](R) reader structure"]
impl crate::Readable for ISOENDPT_CSR0_ISOENDPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isoendpt_csr0_isoendpt::W](W) writer structure"]
impl crate::Writable for ISOENDPT_CSR0_ISOENDPT_SPEC {
    type Writer = W;
}
