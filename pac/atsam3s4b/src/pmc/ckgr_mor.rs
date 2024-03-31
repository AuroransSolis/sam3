#[doc = "Register `CKGR_MOR` reader"]
pub type R = crate::R<CkgrMorSpec>;
#[doc = "Register `CKGR_MOR` writer"]
pub type W = crate::W<CkgrMorSpec>;
#[doc = "Field `MOSCXTEN` reader - Main Crystal Oscillator Enable"]
pub type MoscxtenR = crate::BitReader;
#[doc = "Field `MOSCXTEN` writer - Main Crystal Oscillator Enable"]
pub type MoscxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCXTBY` reader - Main Crystal Oscillator Bypass"]
pub type MoscxtbyR = crate::BitReader;
#[doc = "Field `MOSCXTBY` writer - Main Crystal Oscillator Bypass"]
pub type MoscxtbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCRCEN` reader - Main On-Chip RC Oscillator Enable"]
pub type MoscrcenR = crate::BitReader;
#[doc = "Field `MOSCRCEN` writer - Main On-Chip RC Oscillator Enable"]
pub type MoscrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Main On-Chip RC Oscillator Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Moscrcf {
    #[doc = "0: The Fast RC Oscillator Frequency is at 4 MHz (default)"]
    _4Mhz = 0,
    #[doc = "1: The Fast RC Oscillator Frequency is at 8 MHz"]
    _8Mhz = 1,
    #[doc = "2: The Fast RC Oscillator Frequency is at 12 MHz"]
    _12Mhz = 2,
}
impl From<Moscrcf> for u8 {
    #[inline(always)]
    fn from(variant: Moscrcf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Moscrcf {
    type Ux = u8;
}
impl crate::IsEnum for Moscrcf {}
#[doc = "Field `MOSCRCF` reader - Main On-Chip RC Oscillator Frequency Selection"]
pub type MoscrcfR = crate::FieldReader<Moscrcf>;
impl MoscrcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Moscrcf> {
        match self.bits {
            0 => Some(Moscrcf::_4Mhz),
            1 => Some(Moscrcf::_8Mhz),
            2 => Some(Moscrcf::_12Mhz),
            _ => None,
        }
    }
    #[doc = "The Fast RC Oscillator Frequency is at 4 MHz (default)"]
    #[inline(always)]
    pub fn is_4_mhz(&self) -> bool {
        *self == Moscrcf::_4Mhz
    }
    #[doc = "The Fast RC Oscillator Frequency is at 8 MHz"]
    #[inline(always)]
    pub fn is_8_mhz(&self) -> bool {
        *self == Moscrcf::_8Mhz
    }
    #[doc = "The Fast RC Oscillator Frequency is at 12 MHz"]
    #[inline(always)]
    pub fn is_12_mhz(&self) -> bool {
        *self == Moscrcf::_12Mhz
    }
}
#[doc = "Field `MOSCRCF` writer - Main On-Chip RC Oscillator Frequency Selection"]
pub type MoscrcfW<'a, REG> = crate::FieldWriter<'a, REG, 3, Moscrcf>;
impl<'a, REG> MoscrcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The Fast RC Oscillator Frequency is at 4 MHz (default)"]
    #[inline(always)]
    pub fn _4_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Moscrcf::_4Mhz)
    }
    #[doc = "The Fast RC Oscillator Frequency is at 8 MHz"]
    #[inline(always)]
    pub fn _8_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Moscrcf::_8Mhz)
    }
    #[doc = "The Fast RC Oscillator Frequency is at 12 MHz"]
    #[inline(always)]
    pub fn _12_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Moscrcf::_12Mhz)
    }
}
#[doc = "Field `MOSCXTST` reader - Main Crystal Oscillator Start-up Time"]
pub type MoscxtstR = crate::FieldReader;
#[doc = "Field `MOSCXTST` writer - Main Crystal Oscillator Start-up Time"]
pub type MoscxtstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "55: Writing any other value in this field aborts the write operation.Always reads as 0."]
    Passwd = 55,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` reader - Write Access Password"]
pub type KeyR = crate::FieldReader<Key>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Key> {
        match self.bits {
            55 => Some(Key::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Key::Passwd
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Passwd)
    }
}
#[doc = "Field `MOSCSEL` reader - Main Oscillator Selection"]
pub type MoscselR = crate::BitReader;
#[doc = "Field `MOSCSEL` writer - Main Oscillator Selection"]
pub type MoscselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CfdenR = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CfdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&self) -> MoscxtenR {
        MoscxtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&self) -> MoscxtbyR {
        MoscxtbyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Main On-Chip RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&self) -> MoscrcenR {
        MoscrcenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Main On-Chip RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&self) -> MoscrcfR {
        MoscrcfR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Start-up Time"]
    #[inline(always)]
    pub fn moscxtst(&self) -> MoscxtstR {
        MoscxtstR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Main Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&self) -> MoscselR {
        MoscselR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CfdenR {
        CfdenR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscxten(&mut self) -> MoscxtenW<CkgrMorSpec> {
        MoscxtenW::new(self, 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn moscxtby(&mut self) -> MoscxtbyW<CkgrMorSpec> {
        MoscxtbyW::new(self, 1)
    }
    #[doc = "Bit 3 - Main On-Chip RC Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcen(&mut self) -> MoscrcenW<CkgrMorSpec> {
        MoscrcenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Main On-Chip RC Oscillator Frequency Selection"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcf(&mut self) -> MoscrcfW<CkgrMorSpec> {
        MoscrcfW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn moscxtst(&mut self) -> MoscxtstW<CkgrMorSpec> {
        MoscxtstW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CkgrMorSpec> {
        KeyW::new(self, 16)
    }
    #[doc = "Bit 24 - Main Oscillator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn moscsel(&mut self) -> MoscselW<CkgrMorSpec> {
        MoscselW::new(self, 24)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CfdenW<CkgrMorSpec> {
        CfdenW::new(self, 25)
    }
}
#[doc = "Main Oscillator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_mor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_mor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgrMorSpec;
impl crate::RegisterSpec for CkgrMorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_mor::R`](R) reader structure"]
impl crate::Readable for CkgrMorSpec {}
#[doc = "`write(|w| ..)` method takes [`ckgr_mor::W`](W) writer structure"]
impl crate::Writable for CkgrMorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CKGR_MOR to value 0x08"]
impl crate::Resettable for CkgrMorSpec {
    const RESET_VALUE: u32 = 0x08;
}
