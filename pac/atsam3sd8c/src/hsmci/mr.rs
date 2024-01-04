#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `CLKDIV` reader - Clock Divider"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock Divider"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWSDIV` reader - Power Saving Divider"]
pub type PWSDIV_R = crate::FieldReader;
#[doc = "Field `PWSDIV` writer - Power Saving Divider"]
pub type PWSDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDPROOF` reader - "]
pub type RDPROOF_R = crate::BitReader;
#[doc = "Field `RDPROOF` writer - "]
pub type RDPROOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPROOF` reader - "]
pub type WRPROOF_R = crate::BitReader;
#[doc = "Field `WRPROOF` writer - "]
pub type WRPROOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBYTE` reader - Force Byte Transfer"]
pub type FBYTE_R = crate::BitReader;
#[doc = "Field `FBYTE` writer - Force Byte Transfer"]
pub type FBYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADV` reader - Padding Value"]
pub type PADV_R = crate::BitReader;
#[doc = "Field `PADV` writer - Padding Value"]
pub type PADV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCMODE` reader - PDC-oriented Mode"]
pub type PDCMODE_R = crate::BitReader;
#[doc = "Field `PDCMODE` writer - PDC-oriented Mode"]
pub type PDCMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    pub fn pwsdiv(&self) -> PWSDIV_R {
        PWSDIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rdproof(&self) -> RDPROOF_R {
        RDPROOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wrproof(&self) -> WRPROOF_R {
        WRPROOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    pub fn fbyte(&self) -> FBYTE_R {
        FBYTE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    pub fn padv(&self) -> PADV_R {
        PADV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDC-oriented Mode"]
    #[inline(always)]
    pub fn pdcmode(&self) -> PDCMODE_R {
        PDCMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<MR_SPEC> {
        CLKDIV_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Power Saving Divider"]
    #[inline(always)]
    #[must_use]
    pub fn pwsdiv(&mut self) -> PWSDIV_W<MR_SPEC> {
        PWSDIV_W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn rdproof(&mut self) -> RDPROOF_W<MR_SPEC> {
        RDPROOF_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn wrproof(&mut self) -> WRPROOF_W<MR_SPEC> {
        WRPROOF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Force Byte Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn fbyte(&mut self) -> FBYTE_W<MR_SPEC> {
        FBYTE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Padding Value"]
    #[inline(always)]
    #[must_use]
    pub fn padv(&mut self) -> PADV_W<MR_SPEC> {
        PADV_W::new(self, 14)
    }
    #[doc = "Bit 15 - PDC-oriented Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pdcmode(&mut self) -> PDCMODE_W<MR_SPEC> {
        PDCMODE_W::new(self, 15)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: u32 = 0;
}
