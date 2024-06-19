#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "CLKA, CLKB Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diva {
    #[doc = "0: CLKA, CLKB clock is turned off"]
    ClkOff = 0,
    #[doc = "1: CLKA, CLKB clock is clock selected by PREA, PREB"]
    ClkDiv1 = 1,
}
impl From<Diva> for u8 {
    #[inline(always)]
    fn from(variant: Diva) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diva {
    type Ux = u8;
}
impl crate::IsEnum for Diva {}
#[doc = "Field `DIVA` reader - CLKA, CLKB Divide Factor"]
pub type DivaR = crate::FieldReader<Diva>;
impl DivaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Diva> {
        match self.bits {
            0 => Some(Diva::ClkOff),
            1 => Some(Diva::ClkDiv1),
            _ => None,
        }
    }
    #[doc = "CLKA, CLKB clock is turned off"]
    #[inline(always)]
    pub fn is_clk_off(&self) -> bool {
        *self == Diva::ClkOff
    }
    #[doc = "CLKA, CLKB clock is clock selected by PREA, PREB"]
    #[inline(always)]
    pub fn is_clk_div1(&self) -> bool {
        *self == Diva::ClkDiv1
    }
}
#[doc = "Field `DIVA` writer - CLKA, CLKB Divide Factor"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 8, Diva>;
impl<'a, REG> DivaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKA, CLKB clock is turned off"]
    #[inline(always)]
    pub fn clk_off(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::ClkOff)
    }
    #[doc = "CLKA, CLKB clock is clock selected by PREA, PREB"]
    #[inline(always)]
    pub fn clk_div1(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::ClkDiv1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prea {
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
}
impl From<Prea> for u8 {
    #[inline(always)]
    fn from(variant: Prea) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prea {
    type Ux = u8;
}
impl crate::IsEnum for Prea {}
#[doc = "Field `PREA` reader - "]
pub type PreaR = crate::FieldReader<Prea>;
impl PreaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prea> {
        match self.bits {
            0 => Some(Prea::Mck),
            1 => Some(Prea::Mckdiv2),
            2 => Some(Prea::Mckdiv4),
            3 => Some(Prea::Mckdiv8),
            4 => Some(Prea::Mckdiv16),
            5 => Some(Prea::Mckdiv32),
            6 => Some(Prea::Mckdiv64),
            7 => Some(Prea::Mckdiv128),
            8 => Some(Prea::Mckdiv256),
            9 => Some(Prea::Mckdiv512),
            10 => Some(Prea::Mckdiv1024),
            _ => None,
        }
    }
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Prea::Mck
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        *self == Prea::Mckdiv2
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        *self == Prea::Mckdiv4
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn is_mckdiv8(&self) -> bool {
        *self == Prea::Mckdiv8
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn is_mckdiv16(&self) -> bool {
        *self == Prea::Mckdiv16
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn is_mckdiv32(&self) -> bool {
        *self == Prea::Mckdiv32
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn is_mckdiv64(&self) -> bool {
        *self == Prea::Mckdiv64
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn is_mckdiv128(&self) -> bool {
        *self == Prea::Mckdiv128
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn is_mckdiv256(&self) -> bool {
        *self == Prea::Mckdiv256
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn is_mckdiv512(&self) -> bool {
        *self == Prea::Mckdiv512
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn is_mckdiv1024(&self) -> bool {
        *self == Prea::Mckdiv1024
    }
}
#[doc = "Field `PREA` writer - "]
pub type PreaW<'a, REG> = crate::FieldWriter<'a, REG, 4, Prea>;
impl<'a, REG> PreaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mck)
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mckdiv2)
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mckdiv4)
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn mckdiv8(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mckdiv8)
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn mckdiv16(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mckdiv16)
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn mckdiv32(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mckdiv32)
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn mckdiv64(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mckdiv64)
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn mckdiv128(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mckdiv128)
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn mckdiv256(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mckdiv256)
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn mckdiv512(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mckdiv512)
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn mckdiv1024(self) -> &'a mut crate::W<REG> {
        self.variant(Prea::Mckdiv1024)
    }
}
#[doc = "CLKA, CLKB Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divb {
    #[doc = "0: CLKA, CLKB clock is turned off"]
    ClkOff = 0,
    #[doc = "1: CLKA, CLKB clock is clock selected by PREA, PREB"]
    ClkDiv1 = 1,
}
impl From<Divb> for u8 {
    #[inline(always)]
    fn from(variant: Divb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divb {
    type Ux = u8;
}
impl crate::IsEnum for Divb {}
#[doc = "Field `DIVB` reader - CLKA, CLKB Divide Factor"]
pub type DivbR = crate::FieldReader<Divb>;
impl DivbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divb> {
        match self.bits {
            0 => Some(Divb::ClkOff),
            1 => Some(Divb::ClkDiv1),
            _ => None,
        }
    }
    #[doc = "CLKA, CLKB clock is turned off"]
    #[inline(always)]
    pub fn is_clk_off(&self) -> bool {
        *self == Divb::ClkOff
    }
    #[doc = "CLKA, CLKB clock is clock selected by PREA, PREB"]
    #[inline(always)]
    pub fn is_clk_div1(&self) -> bool {
        *self == Divb::ClkDiv1
    }
}
#[doc = "Field `DIVB` writer - CLKA, CLKB Divide Factor"]
pub type DivbW<'a, REG> = crate::FieldWriter<'a, REG, 8, Divb>;
impl<'a, REG> DivbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKA, CLKB clock is turned off"]
    #[inline(always)]
    pub fn clk_off(self) -> &'a mut crate::W<REG> {
        self.variant(Divb::ClkOff)
    }
    #[doc = "CLKA, CLKB clock is clock selected by PREA, PREB"]
    #[inline(always)]
    pub fn clk_div1(self) -> &'a mut crate::W<REG> {
        self.variant(Divb::ClkDiv1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Preb {
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
}
impl From<Preb> for u8 {
    #[inline(always)]
    fn from(variant: Preb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Preb {
    type Ux = u8;
}
impl crate::IsEnum for Preb {}
#[doc = "Field `PREB` reader - "]
pub type PrebR = crate::FieldReader<Preb>;
impl PrebR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Preb> {
        match self.bits {
            0 => Some(Preb::Mck),
            1 => Some(Preb::Mckdiv2),
            2 => Some(Preb::Mckdiv4),
            3 => Some(Preb::Mckdiv8),
            4 => Some(Preb::Mckdiv16),
            5 => Some(Preb::Mckdiv32),
            6 => Some(Preb::Mckdiv64),
            7 => Some(Preb::Mckdiv128),
            8 => Some(Preb::Mckdiv256),
            9 => Some(Preb::Mckdiv512),
            10 => Some(Preb::Mckdiv1024),
            _ => None,
        }
    }
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == Preb::Mck
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        *self == Preb::Mckdiv2
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        *self == Preb::Mckdiv4
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn is_mckdiv8(&self) -> bool {
        *self == Preb::Mckdiv8
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn is_mckdiv16(&self) -> bool {
        *self == Preb::Mckdiv16
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn is_mckdiv32(&self) -> bool {
        *self == Preb::Mckdiv32
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn is_mckdiv64(&self) -> bool {
        *self == Preb::Mckdiv64
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn is_mckdiv128(&self) -> bool {
        *self == Preb::Mckdiv128
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn is_mckdiv256(&self) -> bool {
        *self == Preb::Mckdiv256
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn is_mckdiv512(&self) -> bool {
        *self == Preb::Mckdiv512
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn is_mckdiv1024(&self) -> bool {
        *self == Preb::Mckdiv1024
    }
}
#[doc = "Field `PREB` writer - "]
pub type PrebW<'a, REG> = crate::FieldWriter<'a, REG, 4, Preb>;
impl<'a, REG> PrebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Master Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mck)
    }
    #[doc = "Master Clock divided by 2"]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mckdiv2)
    }
    #[doc = "Master Clock divided by 4"]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mckdiv4)
    }
    #[doc = "Master Clock divided by 8"]
    #[inline(always)]
    pub fn mckdiv8(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mckdiv8)
    }
    #[doc = "Master Clock divided by 16"]
    #[inline(always)]
    pub fn mckdiv16(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mckdiv16)
    }
    #[doc = "Master Clock divided by 32"]
    #[inline(always)]
    pub fn mckdiv32(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mckdiv32)
    }
    #[doc = "Master Clock divided by 64"]
    #[inline(always)]
    pub fn mckdiv64(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mckdiv64)
    }
    #[doc = "Master Clock divided by 128"]
    #[inline(always)]
    pub fn mckdiv128(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mckdiv128)
    }
    #[doc = "Master Clock divided by 256"]
    #[inline(always)]
    pub fn mckdiv256(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mckdiv256)
    }
    #[doc = "Master Clock divided by 512"]
    #[inline(always)]
    pub fn mckdiv512(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mckdiv512)
    }
    #[doc = "Master Clock divided by 1024"]
    #[inline(always)]
    pub fn mckdiv1024(self) -> &'a mut crate::W<REG> {
        self.variant(Preb::Mckdiv1024)
    }
}
impl R {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn prea(&self) -> PreaR {
        PreaR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&self) -> DivbR {
        DivbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn preb(&self) -> PrebR {
        PrebR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DivaW<MrSpec> {
        DivaW::new(self, 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn prea(&mut self) -> PreaW<MrSpec> {
        PreaW::new(self, 8)
    }
    #[doc = "Bits 16:23 - CLKA, CLKB Divide Factor"]
    #[inline(always)]
    #[must_use]
    pub fn divb(&mut self) -> DivbW<MrSpec> {
        DivbW::new(self, 16)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn preb(&mut self) -> PrebW<MrSpec> {
        PrebW::new(self, 24)
    }
}
#[doc = "PWM Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0;
}
