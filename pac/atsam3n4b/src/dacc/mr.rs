#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TRGEN_R = crate::BitReader;
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TRGSEL_R = crate::FieldReader<TRGSEL_A>;
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: External trigger"]
    Trgsel0 = 0,
    #[doc = "1: TIO Output of the Timer Counter Channel 0"]
    Trgsel1 = 1,
    #[doc = "2: TIO Output of the Timer Counter Channel 1"]
    Trgsel2 = 2,
    #[doc = "3: TIO Output of the Timer Counter Channel 2"]
    Trgsel3 = 3,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRGSEL_A {
    type Ux = u8;
}
impl TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRGSEL_A> {
        match self.bits {
            0 => Some(TRGSEL_A::Trgsel0),
            1 => Some(TRGSEL_A::Trgsel1),
            2 => Some(TRGSEL_A::Trgsel2),
            3 => Some(TRGSEL_A::Trgsel3),
            _ => None,
        }
    }
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == TRGSEL_A::Trgsel0
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == TRGSEL_A::Trgsel1
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == TRGSEL_A::Trgsel2
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == TRGSEL_A::Trgsel3
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TRGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TRGSEL_A>;
impl<'a, REG> TRGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSEL_A::Trgsel0)
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSEL_A::Trgsel1)
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSEL_A::Trgsel2)
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSEL_A::Trgsel3)
    }
}
#[doc = "Field `DACEN` reader - DAC enable"]
pub type DACEN_R = crate::BitReader;
#[doc = "Field `DACEN` writer - DAC enable"]
pub type DACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORD` reader - Word Transfer"]
pub type WORD_R = crate::BitReader;
#[doc = "Field `WORD` writer - Word Transfer"]
pub type WORD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTUP` reader - Startup Time Selection"]
pub type STARTUP_R = crate::FieldReader;
#[doc = "Field `STARTUP` writer - Startup Time Selection"]
pub type STARTUP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKDIV` reader - DAC Clock Divider for Internal Trigger"]
pub type CLKDIV_R = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - DAC Clock Divider for Internal Trigger"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - DAC enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Word Transfer"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Startup Time Selection"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - DAC Clock Divider for Internal Trigger"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgen(&mut self) -> TRGEN_W<MR_SPEC> {
        TRGEN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TRGSEL_W<MR_SPEC> {
        TRGSEL_W::new(self, 1)
    }
    #[doc = "Bit 4 - DAC enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<MR_SPEC> {
        DACEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Word Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn word(&mut self) -> WORD_W<MR_SPEC> {
        WORD_W::new(self, 5)
    }
    #[doc = "Bits 8:15 - Startup Time Selection"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<MR_SPEC> {
        STARTUP_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - DAC Clock Divider for Internal Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<MR_SPEC> {
        CLKDIV_W::new(self, 16)
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
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
