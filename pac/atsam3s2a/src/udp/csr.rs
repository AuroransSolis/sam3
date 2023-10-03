#[doc = "Register `CSR[%s]` reader"]
pub type R = crate::R<CSR_SPEC>;
#[doc = "Register `CSR[%s]` writer"]
pub type W = crate::W<CSR_SPEC>;
#[doc = "Field `TXCOMP` reader - Generates an IN Packet with Data Previously Written in the DPR"]
pub type TXCOMP_R = crate::BitReader;
#[doc = "Field `TXCOMP` writer - Generates an IN Packet with Data Previously Written in the DPR"]
pub type TXCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_DATA_BK0` reader - Receive Data Bank 0"]
pub type RX_DATA_BK0_R = crate::BitReader;
#[doc = "Field `RX_DATA_BK0` writer - Receive Data Bank 0"]
pub type RX_DATA_BK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSETUP` reader - Received Setup"]
pub type RXSETUP_R = crate::BitReader;
#[doc = "Field `RXSETUP` writer - Received Setup"]
pub type RXSETUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLSENT` reader - Stall Sent"]
pub type STALLSENT_R = crate::BitReader;
#[doc = "Field `STALLSENT` writer - Stall Sent"]
pub type STALLSENT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXPKTRDY` reader - Transmit Packet Ready"]
pub type TXPKTRDY_R = crate::BitReader;
#[doc = "Field `TXPKTRDY` writer - Transmit Packet Ready"]
pub type TXPKTRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCESTALL` reader - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
pub type FORCESTALL_R = crate::BitReader;
#[doc = "Field `FORCESTALL` writer - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
pub type FORCESTALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_DATA_BK1` reader - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
pub type RX_DATA_BK1_R = crate::BitReader;
#[doc = "Field `RX_DATA_BK1` writer - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
pub type RX_DATA_BK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIR` reader - Transfer Direction (only available for control endpoints)"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Transfer Direction (only available for control endpoints)"]
pub type DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<EPTYPE_A>;
#[doc = "Endpoint Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control"]
    Ctrl = 0,
    #[doc = "1: Isochronous OUT"]
    IsoOut = 1,
    #[doc = "2: Bulk OUT"]
    BulkOut = 2,
    #[doc = "3: Interrupt OUT"]
    IntOut = 3,
    #[doc = "5: Isochronous IN"]
    IsoIn = 5,
    #[doc = "6: Bulk IN"]
    BulkIn = 6,
    #[doc = "7: Interrupt IN"]
    IntIn = 7,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPTYPE_A {
    type Ux = u8;
}
impl EPTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPTYPE_A> {
        match self.bits {
            0 => Some(EPTYPE_A::Ctrl),
            1 => Some(EPTYPE_A::IsoOut),
            2 => Some(EPTYPE_A::BulkOut),
            3 => Some(EPTYPE_A::IntOut),
            5 => Some(EPTYPE_A::IsoIn),
            6 => Some(EPTYPE_A::BulkIn),
            7 => Some(EPTYPE_A::IntIn),
            _ => None,
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == EPTYPE_A::Ctrl
    }
    #[doc = "Isochronous OUT"]
    #[inline(always)]
    pub fn is_iso_out(&self) -> bool {
        *self == EPTYPE_A::IsoOut
    }
    #[doc = "Bulk OUT"]
    #[inline(always)]
    pub fn is_bulk_out(&self) -> bool {
        *self == EPTYPE_A::BulkOut
    }
    #[doc = "Interrupt OUT"]
    #[inline(always)]
    pub fn is_int_out(&self) -> bool {
        *self == EPTYPE_A::IntOut
    }
    #[doc = "Isochronous IN"]
    #[inline(always)]
    pub fn is_iso_in(&self) -> bool {
        *self == EPTYPE_A::IsoIn
    }
    #[doc = "Bulk IN"]
    #[inline(always)]
    pub fn is_bulk_in(&self) -> bool {
        *self == EPTYPE_A::BulkIn
    }
    #[doc = "Interrupt IN"]
    #[inline(always)]
    pub fn is_int_in(&self) -> bool {
        *self == EPTYPE_A::IntIn
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EPTYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, EPTYPE_A>;
impl<'a, REG, const O: u8> EPTYPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::Ctrl)
    }
    #[doc = "Isochronous OUT"]
    #[inline(always)]
    pub fn iso_out(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::IsoOut)
    }
    #[doc = "Bulk OUT"]
    #[inline(always)]
    pub fn bulk_out(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::BulkOut)
    }
    #[doc = "Interrupt OUT"]
    #[inline(always)]
    pub fn int_out(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::IntOut)
    }
    #[doc = "Isochronous IN"]
    #[inline(always)]
    pub fn iso_in(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::IsoIn)
    }
    #[doc = "Bulk IN"]
    #[inline(always)]
    pub fn bulk_in(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::BulkIn)
    }
    #[doc = "Interrupt IN"]
    #[inline(always)]
    pub fn int_in(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::IntIn)
    }
}
#[doc = "Field `DTGLE` reader - Data Toggle"]
pub type DTGLE_R = crate::BitReader;
#[doc = "Field `DTGLE` writer - Data Toggle"]
pub type DTGLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPEDS` reader - Endpoint Enable Disable"]
pub type EPEDS_R = crate::BitReader;
#[doc = "Field `EPEDS` writer - Endpoint Enable Disable"]
pub type EPEDS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXBYTECNT` reader - Number of Bytes Available in the FIFO"]
pub type RXBYTECNT_R = crate::FieldReader<u16>;
#[doc = "Field `RXBYTECNT` writer - Number of Bytes Available in the FIFO"]
pub type RXBYTECNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
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
    #[doc = "Bit 3 - Stall Sent"]
    #[inline(always)]
    pub fn stallsent(&self) -> STALLSENT_R {
        STALLSENT_R::new(((self.bits >> 3) & 1) != 0)
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
    #[must_use]
    pub fn txcomp(&mut self) -> TXCOMP_W<CSR_SPEC, 0> {
        TXCOMP_W::new(self)
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_bk0(&mut self) -> RX_DATA_BK0_W<CSR_SPEC, 1> {
        RX_DATA_BK0_W::new(self)
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline(always)]
    #[must_use]
    pub fn rxsetup(&mut self) -> RXSETUP_W<CSR_SPEC, 2> {
        RXSETUP_W::new(self)
    }
    #[doc = "Bit 3 - Stall Sent"]
    #[inline(always)]
    #[must_use]
    pub fn stallsent(&mut self) -> STALLSENT_W<CSR_SPEC, 3> {
        STALLSENT_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline(always)]
    #[must_use]
    pub fn txpktrdy(&mut self) -> TXPKTRDY_W<CSR_SPEC, 4> {
        TXPKTRDY_W::new(self)
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn forcestall(&mut self) -> FORCESTALL_W<CSR_SPEC, 5> {
        FORCESTALL_W::new(self)
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_bk1(&mut self) -> RX_DATA_BK1_W<CSR_SPEC, 6> {
        RX_DATA_BK1_W::new(self)
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CSR_SPEC, 7> {
        DIR_W::new(self)
    }
    #[doc = "Bits 8:10 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<CSR_SPEC, 8> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bit 11 - Data Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn dtgle(&mut self) -> DTGLE_W<CSR_SPEC, 11> {
        DTGLE_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline(always)]
    #[must_use]
    pub fn epeds(&mut self) -> EPEDS_W<CSR_SPEC, 15> {
        EPEDS_W::new(self)
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rxbytecnt(&mut self) -> RXBYTECNT_W<CSR_SPEC, 16> {
        RXBYTECNT_W::new(self)
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
#[doc = "Endpoint Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
