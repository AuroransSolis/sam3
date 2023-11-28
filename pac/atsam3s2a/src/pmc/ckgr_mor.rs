#[doc = "Register `CKGR_MOR` reader"]
pub type R = crate::R<CKGR_MOR_SPEC>;
#[doc = "Register `CKGR_MOR` writer"]
pub type W = crate::W<CKGR_MOR_SPEC>;
#[doc = "Field `MOSCXTEN` reader - Main Crystal Oscillator Enable"]
pub type MOSCXTEN_R = crate::BitReader;
#[doc = "Field `MOSCXTEN` writer - Main Crystal Oscillator Enable"]
pub type MOSCXTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCXTBY` reader - Main Crystal Oscillator Bypass"]
pub type MOSCXTBY_R = crate::BitReader;
#[doc = "Field `MOSCXTBY` writer - Main Crystal Oscillator Bypass"]
pub type MOSCXTBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCRCEN` reader - Main On-Chip RC Oscillator Enable"]
pub type MOSCRCEN_R = crate::BitReader;
#[doc = "Field `MOSCRCEN` writer - Main On-Chip RC Oscillator Enable"]
pub type MOSCRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCRCF` reader - Main On-Chip RC Oscillator Frequency Selection"]
pub type MOSCRCF_R = crate::FieldReader<MOSCRCF_A>;
#[doc = "Main On-Chip RC Oscillator Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MOSCRCF_A {
    #[doc = "0: The Fast RC Oscillator Frequency is at 4 MHz (default)"]
    _4Mhz = 0,
    #[doc = "1: The Fast RC Oscillator Frequency is at 8 MHz"]
    _8Mhz = 1,
    #[doc = "2: The Fast RC Oscillator Frequency is at 12 MHz"]
    _12Mhz = 2,
}
impl From<MOSCRCF_A> for u8 {
    #[inline(always)]
    fn from(variant: MOSCRCF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MOSCRCF_A {
    type Ux = u8;
}
impl MOSCRCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MOSCRCF_A> {
        match self.bits {
            0 => Some(MOSCRCF_A::_4Mhz),
            1 => Some(MOSCRCF_A::_8Mhz),
            2 => Some(MOSCRCF_A::_12Mhz),
            _ => None,
        }
    }
    #[doc = "The Fast RC Oscillator Frequency is at 4 MHz (default)"]
    #[inline(always)]
    pub fn is_4_mhz(&self) -> bool {
        *self == MOSCRCF_A::_4Mhz
    }
    #[doc = "The Fast RC Oscillator Frequency is at 8 MHz"]
    #[inline(always)]
    pub fn is_8_mhz(&self) -> bool {
        *self == MOSCRCF_A::_8Mhz
    }
    #[doc = "The Fast RC Oscillator Frequency is at 12 MHz"]
    #[inline(always)]
    pub fn is_12_mhz(&self) -> bool {
        *self == MOSCRCF_A::_12Mhz
    }
}
#[doc = "Field `MOSCRCF` writer - Main On-Chip RC Oscillator Frequency Selection"]
pub type MOSCRCF_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MOSCRCF_A>;
impl<'a, REG> MOSCRCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The Fast RC Oscillator Frequency is at 4 MHz (default)"]
    #[inline(always)]
    pub fn _4_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MOSCRCF_A::_4Mhz)
    }
    #[doc = "The Fast RC Oscillator Frequency is at 8 MHz"]
    #[inline(always)]
    pub fn _8_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MOSCRCF_A::_8Mhz)
    }
    #[doc = "The Fast RC Oscillator Frequency is at 12 MHz"]
    #[inline(always)]
    pub fn _12_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(MOSCRCF_A::_12Mhz)
    }
}
#[doc = "Field `MOSCXTST` reader - Main Crystal Oscillator Start-up Time"]
pub type MOSCXTST_R = crate::FieldReader;
#[doc = "Field `MOSCXTST` writer - Main Crystal Oscillator Start-up Time"]
pub type MOSCXTST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `KEY` reader - Write Access Password"]
pub type KEY_R = crate::FieldReader<KEY_A>;
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "55: Writing any other value in this field aborts the write operation.Always reads as 0."]
    Passwd = 55,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY_A {
    type Ux = u8;
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            55 => Some(KEY_A::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == KEY_A::Passwd
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY_A>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_A::Passwd)
    }
}
#[doc = "Field `MOSCSEL` reader - Main Oscillator Selection"]
pub type MOSCSEL_R = crate::BitReader;
#[doc = "Field `MOSCSEL` writer - Main Oscillator Selection"]
pub type MOSCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CFDEN_R = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CFDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&self) -> MOSCXTEN_R {
        MOSCXTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&self) -> MOSCXTBY_R {
        MOSCXTBY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Main On-Chip RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&self) -> MOSCRCEN_R {
        MOSCRCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Main On-Chip RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&self) -> MOSCRCF_R {
        MOSCRCF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Start-up Time"]
    #[inline(always)]
    pub fn moscxtst(&self) -> MOSCXTST_R {
        MOSCXTST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Main Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&self) -> MOSCSEL_R {
        MOSCSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscxten(&mut self) -> MOSCXTEN_W<CKGR_MOR_SPEC> {
        MOSCXTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn moscxtby(&mut self) -> MOSCXTBY_W<CKGR_MOR_SPEC> {
        MOSCXTBY_W::new(self, 1)
    }
    #[doc = "Bit 3 - Main On-Chip RC Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcen(&mut self) -> MOSCRCEN_W<CKGR_MOR_SPEC> {
        MOSCRCEN_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Main On-Chip RC Oscillator Frequency Selection"]
    #[inline(always)]
    #[must_use]
    pub fn moscrcf(&mut self) -> MOSCRCF_W<CKGR_MOR_SPEC> {
        MOSCRCF_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn moscxtst(&mut self) -> MOSCXTST_W<CKGR_MOR_SPEC> {
        MOSCXTST_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<CKGR_MOR_SPEC> {
        KEY_W::new(self, 16)
    }
    #[doc = "Bit 24 - Main Oscillator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn moscsel(&mut self) -> MOSCSEL_W<CKGR_MOR_SPEC> {
        MOSCSEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<CKGR_MOR_SPEC> {
        CFDEN_W::new(self, 25)
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
#[doc = "Main Oscillator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ckgr_mor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ckgr_mor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CKGR_MOR_SPEC;
impl crate::RegisterSpec for CKGR_MOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_mor::R`](R) reader structure"]
impl crate::Readable for CKGR_MOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ckgr_mor::W`](W) writer structure"]
impl crate::Writable for CKGR_MOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKGR_MOR to value 0x08"]
impl crate::Resettable for CKGR_MOR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
