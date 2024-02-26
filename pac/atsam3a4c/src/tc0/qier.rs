#[doc = "Register `QIER` writer"]
pub type W = crate::W<QierSpec>;
#[doc = "Field `IDX` writer - InDeX"]
pub type IdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` writer - DIRection CHanGe"]
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QERR` writer - Quadrature ERRor"]
pub type QerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - InDeX"]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IdxW<QierSpec> {
        IdxW::new(self, 0)
    }
    #[doc = "Bit 1 - DIRection CHanGe"]
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DirchgW<QierSpec> {
        DirchgW::new(self, 1)
    }
    #[doc = "Bit 2 - Quadrature ERRor"]
    #[inline(always)]
    #[must_use]
    pub fn qerr(&mut self) -> QerrW<QierSpec> {
        QerrW::new(self, 2)
    }
}
#[doc = "QDEC Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QierSpec;
impl crate::RegisterSpec for QierSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qier::W`](W) writer structure"]
impl crate::Writable for QierSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
