#[doc = "Register `PMC_WPMR` reader"]
pub type R = crate::R<PMC_WPMR_SPEC>;
#[doc = "Register `PMC_WPMR` writer"]
pub type W = crate::W<PMC_WPMR_SPEC>;
#[doc = "Field `WPEN` reader - Write Protect Enable"]
pub type WPEN_R = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protect Enable"]
pub type WPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPKEY` reader - Write Protect KEY"]
pub type WPKEY_R = crate::FieldReader<WPKEY_A>;
#[doc = "Write Protect KEY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WPKEY_A {
    #[doc = "5262659: Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    Passwd = 5262659,
}
impl From<WPKEY_A> for u32 {
    #[inline(always)]
    fn from(variant: WPKEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WPKEY_A {
    type Ux = u32;
}
impl WPKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WPKEY_A> {
        match self.bits {
            5262659 => Some(WPKEY_A::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == WPKEY_A::Passwd
    }
}
#[doc = "Field `WPKEY` writer - Write Protect KEY"]
pub type WPKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 24, WPKEY_A>;
impl<'a, REG> WPKEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(WPKEY_A::Passwd)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WPEN_W<PMC_WPMR_SPEC> {
        WPEN_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<PMC_WPMR_SPEC> {
        WPKEY_W::new(self, 8)
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
#[doc = "Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_wpmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_wpmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_WPMR_SPEC;
impl crate::RegisterSpec for PMC_WPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_wpmr::R`](R) reader structure"]
impl crate::Readable for PMC_WPMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmc_wpmr::W`](W) writer structure"]
impl crate::Writable for PMC_WPMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC_WPMR to value 0"]
impl crate::Resettable for PMC_WPMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
