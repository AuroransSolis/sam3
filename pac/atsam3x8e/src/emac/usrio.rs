#[doc = "Register `USRIO` reader"]
pub type R = crate::R<UsrioSpec>;
#[doc = "Register `USRIO` writer"]
pub type W = crate::W<UsrioSpec>;
#[doc = "Field `RMII` reader - Reduce MII"]
pub type RmiiR = crate::BitReader;
#[doc = "Field `RMII` writer - Reduce MII"]
pub type RmiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - Clock Enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock Enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reduce MII"]
    #[inline(always)]
    pub fn rmii(&self) -> RmiiR {
        RmiiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reduce MII"]
    #[inline(always)]
    #[must_use]
    pub fn rmii(&mut self) -> RmiiW<UsrioSpec> {
        RmiiW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<UsrioSpec> {
        ClkenW::new(self, 1)
    }
}
#[doc = "User Input/Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usrio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usrio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsrioSpec;
impl crate::RegisterSpec for UsrioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usrio::R`](R) reader structure"]
impl crate::Readable for UsrioSpec {}
#[doc = "`write(|w| ..)` method takes [`usrio::W`](W) writer structure"]
impl crate::Writable for UsrioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USRIO to value 0"]
impl crate::Resettable for UsrioSpec {
    const RESET_VALUE: u32 = 0;
}
