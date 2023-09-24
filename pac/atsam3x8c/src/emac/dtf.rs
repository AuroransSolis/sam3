#[doc = "Register `DTF` reader"]
pub type R = crate::R<DTF_SPEC>;
#[doc = "Register `DTF` writer"]
pub type W = crate::W<DTF_SPEC>;
#[doc = "Field `DTF` reader - Deferred Transmission Frames"]
pub type DTF_R = crate::FieldReader<u16>;
#[doc = "Field `DTF` writer - Deferred Transmission Frames"]
pub type DTF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Deferred Transmission Frames"]
    #[inline(always)]
    pub fn dtf(&self) -> DTF_R {
        DTF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Deferred Transmission Frames"]
    #[inline(always)]
    #[must_use]
    pub fn dtf(&mut self) -> DTF_W<DTF_SPEC, 0> {
        DTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Deferred Transmission Frames Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTF_SPEC;
impl crate::RegisterSpec for DTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtf::R`](R) reader structure"]
impl crate::Readable for DTF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtf::W`](W) writer structure"]
impl crate::Writable for DTF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTF to value 0"]
impl crate::Resettable for DTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
