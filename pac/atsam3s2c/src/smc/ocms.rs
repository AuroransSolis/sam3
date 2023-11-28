#[doc = "Register `OCMS` reader"]
pub type R = crate::R<OCMS_SPEC>;
#[doc = "Register `OCMS` writer"]
pub type W = crate::W<OCMS_SPEC>;
#[doc = "Field `SMSE` reader - Static Memory Controller Scrambling Enable"]
pub type SMSE_R = crate::BitReader;
#[doc = "Field `SMSE` writer - Static Memory Controller Scrambling Enable"]
pub type SMSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS0SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS0SE_R = crate::BitReader;
#[doc = "Field `CS0SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS0SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS1SE_R = crate::BitReader;
#[doc = "Field `CS1SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS1SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS2SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS2SE_R = crate::BitReader;
#[doc = "Field `CS2SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS2SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS3SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS3SE_R = crate::BitReader;
#[doc = "Field `CS3SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS3SE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&self) -> SMSE_R {
        SMSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs0se(&self) -> CS0SE_R {
        CS0SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs1se(&self) -> CS1SE_R {
        CS1SE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs2se(&self) -> CS2SE_R {
        CS2SE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs3se(&self) -> CS3SE_R {
        CS3SE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smse(&mut self) -> SMSE_W<OCMS_SPEC> {
        SMSE_W::new(self, 0)
    }
    #[doc = "Bit 16 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs0se(&mut self) -> CS0SE_W<OCMS_SPEC> {
        CS0SE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs1se(&mut self) -> CS1SE_W<OCMS_SPEC> {
        CS1SE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs2se(&mut self) -> CS2SE_W<OCMS_SPEC> {
        CS2SE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs3se(&mut self) -> CS3SE_W<OCMS_SPEC> {
        CS3SE_W::new(self, 19)
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
#[doc = "SMC OCMS MODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocms::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocms::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCMS_SPEC;
impl crate::RegisterSpec for OCMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocms::R`](R) reader structure"]
impl crate::Readable for OCMS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocms::W`](W) writer structure"]
impl crate::Writable for OCMS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCMS to value 0"]
impl crate::Resettable for OCMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
