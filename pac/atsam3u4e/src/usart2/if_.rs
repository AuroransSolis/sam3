#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `IRDA_FILTER` reader - IrDA Filter"]
pub type IrdaFilterR = crate::FieldReader;
#[doc = "Field `IRDA_FILTER` writer - IrDA Filter"]
pub type IrdaFilterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IrDA Filter"]
    #[inline(always)]
    pub fn irda_filter(&self) -> IrdaFilterR {
        IrdaFilterR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA Filter"]
    #[inline(always)]
    #[must_use]
    pub fn irda_filter(&mut self) -> IrdaFilterW<IfSpec> {
        IrdaFilterW::new(self, 0)
    }
}
#[doc = "IrDA Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {
    const RESET_VALUE: u32 = 0;
}
