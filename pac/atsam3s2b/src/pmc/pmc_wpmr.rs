#[doc = "Register `PMC_WPMR` reader"]
pub type R = crate::R<PmcWpmrSpec>;
#[doc = "Register `PMC_WPMR` writer"]
pub type W = crate::W<PmcWpmrSpec>;
#[doc = "Field `WPEN` reader - Write Protect Enable"]
pub type WpenR = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protect Enable"]
pub type WpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Protect KEY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Wpkey {
    #[doc = "5262659: Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    Passwd = 5262659,
}
impl From<Wpkey> for u32 {
    #[inline(always)]
    fn from(variant: Wpkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wpkey {
    type Ux = u32;
}
impl crate::IsEnum for Wpkey {}
#[doc = "Field `WPKEY` reader - Write Protect KEY"]
pub type WpkeyR = crate::FieldReader<Wpkey>;
impl WpkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wpkey> {
        match self.bits {
            5262659 => Some(Wpkey::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Wpkey::Passwd
    }
}
#[doc = "Field `WPKEY` writer - Write Protect KEY"]
pub type WpkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Wpkey>;
impl<'a, REG> WpkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Wpkey::Passwd)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WpenR {
        WpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY"]
    #[inline(always)]
    pub fn wpkey(&self) -> WpkeyR {
        WpkeyR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WpenW<PmcWpmrSpec> {
        WpenW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WpkeyW<PmcWpmrSpec> {
        WpkeyW::new(self, 8)
    }
}
#[doc = "Write Protect Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_wpmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_wpmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcWpmrSpec;
impl crate::RegisterSpec for PmcWpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_wpmr::R`](R) reader structure"]
impl crate::Readable for PmcWpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmc_wpmr::W`](W) writer structure"]
impl crate::Writable for PmcWpmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMC_WPMR to value 0"]
impl crate::Resettable for PmcWpmrSpec {
    const RESET_VALUE: u32 = 0;
}
