#[doc = "Register `MDR` reader"]
pub type R = crate::R<MDR_SPEC>;
#[doc = "Register `MDR` writer"]
pub type W = crate::W<MDR_SPEC>;
#[doc = "Field `MD` reader - Memory Device Type"]
pub type MD_R = crate::FieldReader<MD_A>;
#[doc = "Memory Device Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MD_A {
    #[doc = "0: SDRAM"]
    Sdram = 0,
    #[doc = "1: Low-power SDRAM"]
    Lpsdram = 1,
}
impl From<MD_A> for u8 {
    #[inline(always)]
    fn from(variant: MD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MD_A {
    type Ux = u8;
}
impl MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MD_A> {
        match self.bits {
            0 => Some(MD_A::Sdram),
            1 => Some(MD_A::Lpsdram),
            _ => None,
        }
    }
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn is_sdram(&self) -> bool {
        *self == MD_A::Sdram
    }
    #[doc = "Low-power SDRAM"]
    #[inline(always)]
    pub fn is_lpsdram(&self) -> bool {
        *self == MD_A::Lpsdram
    }
}
#[doc = "Field `MD` writer - Memory Device Type"]
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MD_A>;
impl<'a, REG> MD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn sdram(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::Sdram)
    }
    #[doc = "Low-power SDRAM"]
    #[inline(always)]
    pub fn lpsdram(self) -> &'a mut crate::W<REG> {
        self.variant(MD_A::Lpsdram)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<MDR_SPEC> {
        MD_W::new(self, 0)
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
#[doc = "SDRAMC Memory Device Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDR_SPEC;
impl crate::RegisterSpec for MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdr::R`](R) reader structure"]
impl crate::Readable for MDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdr::W`](W) writer structure"]
impl crate::Writable for MDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
