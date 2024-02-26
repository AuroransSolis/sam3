#[doc = "Register `FIFO185` reader"]
pub type R = crate::R<Fifo185Spec>;
#[doc = "Register `FIFO185` writer"]
pub type W = crate::W<Fifo185Spec>;
#[doc = "Field `DATA` reader - Data to Read or Data to Write"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data to Read or Data to Write"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data to Read or Data to Write"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to Read or Data to Write"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Fifo185Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "FIFO Memory Aperture0 185\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo185::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo185::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo185Spec;
impl crate::RegisterSpec for Fifo185Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo185::R`](R) reader structure"]
impl crate::Readable for Fifo185Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo185::W`](W) writer structure"]
impl crate::Writable for Fifo185Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
