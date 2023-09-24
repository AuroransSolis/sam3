#[doc = "Register `NSR` reader"]
pub type R = crate::R<NSR_SPEC>;
#[doc = "Field `MDIO` reader - "]
pub type MDIO_R = crate::BitReader;
#[doc = "Field `IDLE` reader - "]
pub type IDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mdio(&self) -> MDIO_R {
        MDIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Network Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSR_SPEC;
impl crate::RegisterSpec for NSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsr::R`](R) reader structure"]
impl crate::Readable for NSR_SPEC {}
