#[doc = "Register `CMR0` reader"]
pub type R = crate::R<CMR0_SPEC>;
#[doc = "Register `CMR0` writer"]
pub type W = crate::W<CMR0_SPEC>;
#[doc = "Field `CPRE` reader - Channel Pre-scaler"]
pub type CPRE_R = crate::FieldReader<CPRE_A>;
#[doc = "Channel Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPRE_A {
    #[doc = "0: Master Clock"]
    Mck = 0,
    #[doc = "1: Master Clock divided by 2"]
    Mckdiv2 = 1,
    #[doc = "2: Master Clock divided by 4"]
    Mckdiv4 = 2,
    #[doc = "3: Master Clock divided by 8"]
    Mckdiv8 = 3,
    #[doc = "4: Master Clock divided by 16"]
    Mckdiv16 = 4,
    #[doc = "5: Master Clock divided by 32"]
    Mckdiv32 = 5,
    #[doc = "6: Master Clock divided by 64"]
    Mckdiv64 = 6,
    #[doc = "7: Master Clock divided by 128"]
    Mckdiv128 = 7,
    #[doc = "8: Master Clock divided by 256"]
    Mckdiv256 = 8,
    #[doc = "9: Master Clock divided by 512"]
    Mckdiv512 = 9,
    #[doc = "10: Master Clock divided by 1024"]
    Mckdiv1024 = 10,
    #[doc = "11: Clock A"]
    Clka = 11,
    #[doc = "12: Clock B"]
    Clkb = 12,
}
impl From<CPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPRE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CPRE_A {
    type Ux = u8;
}
impl CPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CPRE_A> {
        match self.bits {
            0 => Some(CPRE_A::Mck),
            1 => Some(CPRE_A::Mckdiv2),
            2 => Some(CPRE_A::Mckdiv4),
            3 => Some(CPRE_A::Mckdiv8),
            4 => Some(CPRE_A::Mckdiv16),
            5 => Some(CPRE_A::Mckdiv32),
            6 => Some(CPRE_A::Mckdiv64),
            7 => Some(CPRE_A::Mckdiv128),
            8 => Some(CPRE_A::Mckdiv256),
            9 => Some(CPRE_A::Mckdiv512),
            10 => Some(CPRE_A::Mckdiv1024),
            11 => Some(CPRE_A::Clka),
            12 => Some(CPRE_A::Clkb),
            _ => None,
        }
    }
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CPRE_A::Mck
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        *self == CPRE_A::Mckdiv2
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        *self == CPRE_A::Mckdiv4
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn is_mckdiv8(&self) -> bool {
        *self == CPRE_A::Mckdiv8
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn is_mckdiv16(&self) -> bool {
        *self == CPRE_A::Mckdiv16
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn is_mckdiv32(&self) -> bool {
        *self == CPRE_A::Mckdiv32
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn is_mckdiv64(&self) -> bool {
        *self == CPRE_A::Mckdiv64
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn is_mckdiv128(&self) -> bool {
        *self == CPRE_A::Mckdiv128
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn is_mckdiv256(&self) -> bool {
        *self == CPRE_A::Mckdiv256
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn is_mckdiv512(&self) -> bool {
        *self == CPRE_A::Mckdiv512
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn is_mckdiv1024(&self) -> bool {
        *self == CPRE_A::Mckdiv1024
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn is_clka(&self) -> bool {
        *self == CPRE_A::Clka
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn is_clkb(&self) -> bool {
        *self == CPRE_A::Clkb
    }
}
#[doc = "Field `CPRE` writer - Channel Pre-scaler"]
pub type CPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CPRE_A>;
impl<'a, REG> CPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mck)
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mckdiv2)
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mckdiv4)
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn mckdiv8(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mckdiv8)
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn mckdiv16(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mckdiv16)
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn mckdiv32(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mckdiv32)
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn mckdiv64(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mckdiv64)
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn mckdiv128(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mckdiv128)
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn mckdiv256(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mckdiv256)
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn mckdiv512(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mckdiv512)
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn mckdiv1024(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Mckdiv1024)
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn clka(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Clka)
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn clkb(self) -> &'a mut crate::W<REG> {
        self.variant(CPRE_A::Clkb)
    }
}
#[doc = "Field `CALG` reader - Channel Alignment"]
pub type CALG_R = crate::BitReader;
#[doc = "Field `CALG` writer - Channel Alignment"]
pub type CALG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Channel Polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - Channel Polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPD` reader - Channel Update Period"]
pub type CPD_R = crate::BitReader;
#[doc = "Field `CPD` writer - Channel Update Period"]
pub type CPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&self) -> CPRE_R {
        CPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&self) -> CALG_R {
        CALG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Update Period"]
    #[inline(always)]
    pub fn cpd(&self) -> CPD_R {
        CPD_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    #[must_use]
    pub fn cpre(&mut self) -> CPRE_W<CMR0_SPEC> {
        CPRE_W::new(self, 0)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn calg(&mut self) -> CALG_W<CMR0_SPEC> {
        CALG_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CMR0_SPEC> {
        CPOL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel Update Period"]
    #[inline(always)]
    #[must_use]
    pub fn cpd(&mut self) -> CPD_W<CMR0_SPEC> {
        CPD_W::new(self, 10)
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
#[doc = "PWM Channel Mode Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMR0_SPEC;
impl crate::RegisterSpec for CMR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmr0::R`](R) reader structure"]
impl crate::Readable for CMR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmr0::W`](W) writer structure"]
impl crate::Writable for CMR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMR0 to value 0"]
impl crate::Resettable for CMR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
