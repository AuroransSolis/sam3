#[doc = "Register `QIDR` writer"]
pub type W = crate::W<QidrSpec>;
#[doc = "Field `IDX` writer - Index"]
pub type IdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` writer - Direction Change"]
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QERR` writer - Quadrature Error"]
pub type QerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    #[must_use]
    pub fn idx(&mut self) -> IdxW<QidrSpec> {
        IdxW::new(self, 0)
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DirchgW<QidrSpec> {
        DirchgW::new(self, 1)
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    #[must_use]
    pub fn qerr(&mut self) -> QerrW<QidrSpec> {
        QerrW::new(self, 2)
    }
}
#[doc = "QDEC Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QidrSpec;
impl crate::RegisterSpec for QidrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qidr::W`](W) writer structure"]
impl crate::Writable for QidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
