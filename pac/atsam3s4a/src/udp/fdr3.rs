#[doc = "Register `FDR3` reader"]
pub type R = crate::R<Fdr3Spec>;
#[doc = "Register `FDR3` writer"]
pub type W = crate::W<Fdr3Spec>;
#[doc = "Field `FIFO_DATA` reader - FIFO Data Value"]
pub type FifoDataR = crate::FieldReader;
#[doc = "Field `FIFO_DATA` writer - FIFO Data Value"]
pub type FifoDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FIFO Data Value"]
    #[inline(always)]
    pub fn fifo_data(&self) -> FifoDataR {
        FifoDataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_data(&mut self) -> FifoDataW<Fdr3Spec> {
        FifoDataW::new(self, 0)
    }
}
#[doc = "Endpoint FIFO Data Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`fdr3::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fdr3Spec;
impl crate::RegisterSpec for Fdr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr3::R`](R) reader structure"]
impl crate::Readable for Fdr3Spec {}
#[doc = "`write(|w| ..)` method takes [`fdr3::W`](W) writer structure"]
impl crate::Writable for Fdr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
