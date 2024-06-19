#[doc = "Register `GCFG` reader"]
pub type R = crate::R<GcfgSpec>;
#[doc = "Register `GCFG` writer"]
pub type W = crate::W<GcfgSpec>;
#[doc = "Arbiter Configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArbCfg {
    #[doc = "0: Fixed priority arbiter (see \"Basic Definitions\" )"]
    Fixed = 0,
    #[doc = "1: Modified round robin arbiter."]
    RoundRobin = 1,
}
impl From<ArbCfg> for bool {
    #[inline(always)]
    fn from(variant: ArbCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARB_CFG` reader - Arbiter Configuration"]
pub type ArbCfgR = crate::BitReader<ArbCfg>;
impl ArbCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ArbCfg {
        match self.bits {
            false => ArbCfg::Fixed,
            true => ArbCfg::RoundRobin,
        }
    }
    #[doc = "Fixed priority arbiter (see \"Basic Definitions\" )"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == ArbCfg::Fixed
    }
    #[doc = "Modified round robin arbiter."]
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        *self == ArbCfg::RoundRobin
    }
}
#[doc = "Field `ARB_CFG` writer - Arbiter Configuration"]
pub type ArbCfgW<'a, REG> = crate::BitWriter<'a, REG, ArbCfg>;
impl<'a, REG> ArbCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed priority arbiter (see \"Basic Definitions\" )"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(ArbCfg::Fixed)
    }
    #[doc = "Modified round robin arbiter."]
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut crate::W<REG> {
        self.variant(ArbCfg::RoundRobin)
    }
}
impl R {
    #[doc = "Bit 4 - Arbiter Configuration"]
    #[inline(always)]
    pub fn arb_cfg(&self) -> ArbCfgR {
        ArbCfgR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Arbiter Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn arb_cfg(&mut self) -> ArbCfgW<GcfgSpec> {
        ArbCfgW::new(self, 4)
    }
}
#[doc = "DMAC Global Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcfgSpec;
impl crate::RegisterSpec for GcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcfg::R`](R) reader structure"]
impl crate::Readable for GcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gcfg::W`](W) writer structure"]
impl crate::Writable for GcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCFG to value 0x10"]
impl crate::Resettable for GcfgSpec {
    const RESET_VALUE: u32 = 0x10;
}
