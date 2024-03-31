#[doc = "Register `CMR3` reader"]
pub type R = crate::R<Cmr3Spec>;
#[doc = "Register `CMR3` writer"]
pub type W = crate::W<Cmr3Spec>;
#[doc = "Channel Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpre {
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
impl From<Cpre> for u8 {
    #[inline(always)]
    fn from(variant: Cpre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpre {
    type Ux = u8;
}
impl crate::IsEnum for Cpre {}
#[doc = "Field `CPRE` reader - Channel Pre-scaler"]
pub type CpreR = crate::FieldReader<Cpre>;
impl CpreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpre> {
        match self.bits {
            0 => Some(Cpre::Mck),
            1 => Some(Cpre::Mckdiv2),
            2 => Some(Cpre::Mckdiv4),
            3 => Some(Cpre::Mckdiv8),
            4 => Some(Cpre::Mckdiv16),
            5 => Some(Cpre::Mckdiv32),
            6 => Some(Cpre::Mckdiv64),
            7 => Some(Cpre::Mckdiv128),
            8 => Some(Cpre::Mckdiv256),
            9 => Some(Cpre::Mckdiv512),
            10 => Some(Cpre::Mckdiv1024),
            11 => Some(Cpre::Clka),
            12 => Some(Cpre::Clkb),
            _ => None,
        }
    }
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Cpre::Mck
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        *self == Cpre::Mckdiv2
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        *self == Cpre::Mckdiv4
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn is_mckdiv8(&self) -> bool {
        *self == Cpre::Mckdiv8
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn is_mckdiv16(&self) -> bool {
        *self == Cpre::Mckdiv16
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn is_mckdiv32(&self) -> bool {
        *self == Cpre::Mckdiv32
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn is_mckdiv64(&self) -> bool {
        *self == Cpre::Mckdiv64
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn is_mckdiv128(&self) -> bool {
        *self == Cpre::Mckdiv128
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn is_mckdiv256(&self) -> bool {
        *self == Cpre::Mckdiv256
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn is_mckdiv512(&self) -> bool {
        *self == Cpre::Mckdiv512
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn is_mckdiv1024(&self) -> bool {
        *self == Cpre::Mckdiv1024
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn is_clka(&self) -> bool {
        *self == Cpre::Clka
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn is_clkb(&self) -> bool {
        *self == Cpre::Clkb
    }
}
#[doc = "Field `CPRE` writer - Channel Pre-scaler"]
pub type CpreW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cpre>;
impl<'a, REG> CpreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mck)
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mckdiv2)
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mckdiv4)
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn mckdiv8(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mckdiv8)
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn mckdiv16(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mckdiv16)
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn mckdiv32(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mckdiv32)
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn mckdiv64(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mckdiv64)
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn mckdiv128(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mckdiv128)
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn mckdiv256(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mckdiv256)
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn mckdiv512(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mckdiv512)
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn mckdiv1024(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Mckdiv1024)
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn clka(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Clka)
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn clkb(self) -> &'a mut crate::W<REG> {
        self.variant(Cpre::Clkb)
    }
}
#[doc = "Field `CALG` reader - Channel Alignment"]
pub type CalgR = crate::BitReader;
#[doc = "Field `CALG` writer - Channel Alignment"]
pub type CalgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Channel Polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Channel Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPD` reader - Channel Update Period"]
pub type CpdR = crate::BitReader;
#[doc = "Field `CPD` writer - Channel Update Period"]
pub type CpdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&self) -> CpreR {
        CpreR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&self) -> CalgR {
        CalgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Update Period"]
    #[inline(always)]
    pub fn cpd(&self) -> CpdR {
        CpdR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    #[must_use]
    pub fn cpre(&mut self) -> CpreW<Cmr3Spec> {
        CpreW::new(self, 0)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    #[must_use]
    pub fn calg(&mut self) -> CalgW<Cmr3Spec> {
        CalgW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<Cmr3Spec> {
        CpolW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel Update Period"]
    #[inline(always)]
    #[must_use]
    pub fn cpd(&mut self) -> CpdW<Cmr3Spec> {
        CpdW::new(self, 10)
    }
}
#[doc = "PWM Channel Mode Register (ch_num = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmr3Spec;
impl crate::RegisterSpec for Cmr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmr3::R`](R) reader structure"]
impl crate::Readable for Cmr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cmr3::W`](W) writer structure"]
impl crate::Writable for Cmr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMR3 to value 0"]
impl crate::Resettable for Cmr3Spec {
    const RESET_VALUE: u32 = 0;
}
