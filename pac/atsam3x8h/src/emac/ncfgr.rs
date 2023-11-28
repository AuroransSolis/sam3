#[doc = "Register `NCFGR` reader"]
pub type R = crate::R<NCFGR_SPEC>;
#[doc = "Register `NCFGR` writer"]
pub type W = crate::W<NCFGR_SPEC>;
#[doc = "Field `SPD` reader - Speed"]
pub type SPD_R = crate::BitReader;
#[doc = "Field `SPD` writer - Speed"]
pub type SPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FD` reader - Full Duplex"]
pub type FD_R = crate::BitReader;
#[doc = "Field `FD` writer - Full Duplex"]
pub type FD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JFRAME` reader - Jumbo Frames"]
pub type JFRAME_R = crate::BitReader;
#[doc = "Field `JFRAME` writer - Jumbo Frames"]
pub type JFRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAF` reader - Copy All Frames"]
pub type CAF_R = crate::BitReader;
#[doc = "Field `CAF` writer - Copy All Frames"]
pub type CAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBC` reader - No Broadcast"]
pub type NBC_R = crate::BitReader;
#[doc = "Field `NBC` writer - No Broadcast"]
pub type NBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTI` reader - Multicast Hash Enable"]
pub type MTI_R = crate::BitReader;
#[doc = "Field `MTI` writer - Multicast Hash Enable"]
pub type MTI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNI` reader - Unicast Hash Enable"]
pub type UNI_R = crate::BitReader;
#[doc = "Field `UNI` writer - Unicast Hash Enable"]
pub type UNI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIG` reader - Receive 1536 bytes frames"]
pub type BIG_R = crate::BitReader;
#[doc = "Field `BIG` writer - Receive 1536 bytes frames"]
pub type BIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK` reader - MDC clock divider"]
pub type CLK_R = crate::FieldReader<CLK_A>;
#[doc = "MDC clock divider\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_A {
    #[doc = "0: MCK divided by 8 (MCK up to 20 MHz)."]
    Mck8 = 0,
    #[doc = "1: MCK divided by 16 (MCK up to 40 MHz)."]
    Mck16 = 1,
    #[doc = "2: MCK divided by 32 (MCK up to 80 MHz)."]
    Mck32 = 2,
    #[doc = "3: MCK divided by 64 (MCK up to 160 MHz)."]
    Mck64 = 3,
}
impl From<CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK_A {
    type Ux = u8;
}
impl CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLK_A {
        match self.bits {
            0 => CLK_A::Mck8,
            1 => CLK_A::Mck16,
            2 => CLK_A::Mck32,
            3 => CLK_A::Mck64,
            _ => unreachable!(),
        }
    }
    #[doc = "MCK divided by 8 (MCK up to 20 MHz)."]
    #[inline(always)]
    pub fn is_mck_8(&self) -> bool {
        *self == CLK_A::Mck8
    }
    #[doc = "MCK divided by 16 (MCK up to 40 MHz)."]
    #[inline(always)]
    pub fn is_mck_16(&self) -> bool {
        *self == CLK_A::Mck16
    }
    #[doc = "MCK divided by 32 (MCK up to 80 MHz)."]
    #[inline(always)]
    pub fn is_mck_32(&self) -> bool {
        *self == CLK_A::Mck32
    }
    #[doc = "MCK divided by 64 (MCK up to 160 MHz)."]
    #[inline(always)]
    pub fn is_mck_64(&self) -> bool {
        *self == CLK_A::Mck64
    }
}
#[doc = "Field `CLK` writer - MDC clock divider"]
pub type CLK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CLK_A>;
impl<'a, REG> CLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCK divided by 8 (MCK up to 20 MHz)."]
    #[inline(always)]
    pub fn mck_8(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_A::Mck8)
    }
    #[doc = "MCK divided by 16 (MCK up to 40 MHz)."]
    #[inline(always)]
    pub fn mck_16(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_A::Mck16)
    }
    #[doc = "MCK divided by 32 (MCK up to 80 MHz)."]
    #[inline(always)]
    pub fn mck_32(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_A::Mck32)
    }
    #[doc = "MCK divided by 64 (MCK up to 160 MHz)."]
    #[inline(always)]
    pub fn mck_64(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_A::Mck64)
    }
}
#[doc = "Field `RTY` reader - Retry test"]
pub type RTY_R = crate::BitReader;
#[doc = "Field `RTY` writer - Retry test"]
pub type RTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAE` reader - Pause Enable"]
pub type PAE_R = crate::BitReader;
#[doc = "Field `PAE` writer - Pause Enable"]
pub type PAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBOF` reader - Receive Buffer Offset"]
pub type RBOF_R = crate::FieldReader<RBOF_A>;
#[doc = "Receive Buffer Offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RBOF_A {
    #[doc = "0: No offset from start of receive buffer."]
    Offset0 = 0,
    #[doc = "1: One-byte offset from start of receive buffer."]
    Offset1 = 1,
    #[doc = "2: Two-byte offset from start of receive buffer."]
    Offset2 = 2,
    #[doc = "3: Three-byte offset from start of receive buffer."]
    Offset3 = 3,
}
impl From<RBOF_A> for u8 {
    #[inline(always)]
    fn from(variant: RBOF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RBOF_A {
    type Ux = u8;
}
impl RBOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBOF_A {
        match self.bits {
            0 => RBOF_A::Offset0,
            1 => RBOF_A::Offset1,
            2 => RBOF_A::Offset2,
            3 => RBOF_A::Offset3,
            _ => unreachable!(),
        }
    }
    #[doc = "No offset from start of receive buffer."]
    #[inline(always)]
    pub fn is_offset_0(&self) -> bool {
        *self == RBOF_A::Offset0
    }
    #[doc = "One-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn is_offset_1(&self) -> bool {
        *self == RBOF_A::Offset1
    }
    #[doc = "Two-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn is_offset_2(&self) -> bool {
        *self == RBOF_A::Offset2
    }
    #[doc = "Three-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn is_offset_3(&self) -> bool {
        *self == RBOF_A::Offset3
    }
}
#[doc = "Field `RBOF` writer - Receive Buffer Offset"]
pub type RBOF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RBOF_A>;
impl<'a, REG> RBOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_0(self) -> &'a mut crate::W<REG> {
        self.variant(RBOF_A::Offset0)
    }
    #[doc = "One-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_1(self) -> &'a mut crate::W<REG> {
        self.variant(RBOF_A::Offset1)
    }
    #[doc = "Two-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_2(self) -> &'a mut crate::W<REG> {
        self.variant(RBOF_A::Offset2)
    }
    #[doc = "Three-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_3(self) -> &'a mut crate::W<REG> {
        self.variant(RBOF_A::Offset3)
    }
}
#[doc = "Field `RLCE` reader - Receive Length field Checking Enable"]
pub type RLCE_R = crate::BitReader;
#[doc = "Field `RLCE` writer - Receive Length field Checking Enable"]
pub type RLCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRFCS` reader - Discard Receive FCS"]
pub type DRFCS_R = crate::BitReader;
#[doc = "Field `DRFCS` writer - Discard Receive FCS"]
pub type DRFCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFRHD` reader - "]
pub type EFRHD_R = crate::BitReader;
#[doc = "Field `EFRHD` writer - "]
pub type EFRHD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRXFCS` reader - Ignore RX FCS"]
pub type IRXFCS_R = crate::BitReader;
#[doc = "Field `IRXFCS` writer - Ignore RX FCS"]
pub type IRXFCS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&self) -> SPD_R {
        SPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&self) -> FD_R {
        FD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Jumbo Frames"]
    #[inline(always)]
    pub fn jframe(&self) -> JFRAME_R {
        JFRAME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&self) -> CAF_R {
        CAF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&self) -> NBC_R {
        NBC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mti(&self) -> MTI_R {
        MTI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn uni(&self) -> UNI_R {
        UNI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive 1536 bytes frames"]
    #[inline(always)]
    pub fn big(&self) -> BIG_R {
        BIG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - MDC clock divider"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn rty(&self) -> RTY_R {
        RTY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pae(&self) -> PAE_R {
        PAE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rbof(&self) -> RBOF_R {
        RBOF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Receive Length field Checking Enable"]
    #[inline(always)]
    pub fn rlce(&self) -> RLCE_R {
        RLCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Discard Receive FCS"]
    #[inline(always)]
    pub fn drfcs(&self) -> DRFCS_R {
        DRFCS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn efrhd(&self) -> EFRHD_R {
        EFRHD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&self) -> IRXFCS_R {
        IRXFCS_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SPD_W<NCFGR_SPEC> {
        SPD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    #[must_use]
    pub fn fd(&mut self) -> FD_W<NCFGR_SPEC> {
        FD_W::new(self, 1)
    }
    #[doc = "Bit 3 - Jumbo Frames"]
    #[inline(always)]
    #[must_use]
    pub fn jframe(&mut self) -> JFRAME_W<NCFGR_SPEC> {
        JFRAME_W::new(self, 3)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    #[must_use]
    pub fn caf(&mut self) -> CAF_W<NCFGR_SPEC> {
        CAF_W::new(self, 4)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    #[must_use]
    pub fn nbc(&mut self) -> NBC_W<NCFGR_SPEC> {
        NBC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mti(&mut self) -> MTI_W<NCFGR_SPEC> {
        MTI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uni(&mut self) -> UNI_W<NCFGR_SPEC> {
        UNI_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive 1536 bytes frames"]
    #[inline(always)]
    #[must_use]
    pub fn big(&mut self) -> BIG_W<NCFGR_SPEC> {
        BIG_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - MDC clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> CLK_W<NCFGR_SPEC> {
        CLK_W::new(self, 10)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    #[must_use]
    pub fn rty(&mut self) -> RTY_W<NCFGR_SPEC> {
        RTY_W::new(self, 12)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pae(&mut self) -> PAE_W<NCFGR_SPEC> {
        PAE_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    #[must_use]
    pub fn rbof(&mut self) -> RBOF_W<NCFGR_SPEC> {
        RBOF_W::new(self, 14)
    }
    #[doc = "Bit 16 - Receive Length field Checking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rlce(&mut self) -> RLCE_W<NCFGR_SPEC> {
        RLCE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Discard Receive FCS"]
    #[inline(always)]
    #[must_use]
    pub fn drfcs(&mut self) -> DRFCS_W<NCFGR_SPEC> {
        DRFCS_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn efrhd(&mut self) -> EFRHD_W<NCFGR_SPEC> {
        EFRHD_W::new(self, 18)
    }
    #[doc = "Bit 19 - Ignore RX FCS"]
    #[inline(always)]
    #[must_use]
    pub fn irxfcs(&mut self) -> IRXFCS_W<NCFGR_SPEC> {
        IRXFCS_W::new(self, 19)
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
#[doc = "Network Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NCFGR_SPEC;
impl crate::RegisterSpec for NCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncfgr::R`](R) reader structure"]
impl crate::Readable for NCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ncfgr::W`](W) writer structure"]
impl crate::Writable for NCFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NCFGR to value 0x0800"]
impl crate::Resettable for NCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
