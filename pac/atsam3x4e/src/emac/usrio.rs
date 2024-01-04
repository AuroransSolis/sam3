#[doc = "Register `USRIO` reader"]
pub type R = crate::R<USRIO_SPEC>;
#[doc = "Register `USRIO` writer"]
pub type W = crate::W<USRIO_SPEC>;
#[doc = "Field `RMII` reader - Reduce MII"]
pub type RMII_R = crate::BitReader;
#[doc = "Field `RMII` writer - Reduce MII"]
pub type RMII_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - Clock Enable"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock Enable"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reduce MII"]
    #[inline(always)]
    pub fn rmii(&self) -> RMII_R {
        RMII_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reduce MII"]
    #[inline(always)]
    #[must_use]
    pub fn rmii(&mut self) -> RMII_W<USRIO_SPEC> {
        RMII_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<USRIO_SPEC> {
        CLKEN_W::new(self, 1)
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
#[doc = "User Input/Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usrio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usrio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USRIO_SPEC;
impl crate::RegisterSpec for USRIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usrio::R`](R) reader structure"]
impl crate::Readable for USRIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usrio::W`](W) writer structure"]
impl crate::Writable for USRIO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USRIO to value 0"]
impl crate::Resettable for USRIO_SPEC {
    const RESET_VALUE: u32 = 0;
}
