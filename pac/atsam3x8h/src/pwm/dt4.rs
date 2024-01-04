#[doc = "Register `DT4` reader"]
pub type R = crate::R<DT4_SPEC>;
#[doc = "Register `DT4` writer"]
pub type W = crate::W<DT4_SPEC>;
#[doc = "Field `DTH` reader - Dead-Time Value for PWMHx Output"]
pub type DTH_R = crate::FieldReader<u16>;
#[doc = "Field `DTH` writer - Dead-Time Value for PWMHx Output"]
pub type DTH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DTL` reader - Dead-Time Value for PWMLx Output"]
pub type DTL_R = crate::FieldReader<u16>;
#[doc = "Field `DTL` writer - Dead-Time Value for PWMLx Output"]
pub type DTL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Dead-Time Value for PWMHx Output"]
    #[inline(always)]
    pub fn dth(&self) -> DTH_R {
        DTH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    pub fn dtl(&self) -> DTL_R {
        DTL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value for PWMHx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dth(&mut self) -> DTH_W<DT4_SPEC> {
        DTH_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    #[must_use]
    pub fn dtl(&mut self) -> DTL_W<DT4_SPEC> {
        DTL_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM Channel Dead Time Register (ch_num = 4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT4_SPEC;
impl crate::RegisterSpec for DT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt4::R`](R) reader structure"]
impl crate::Readable for DT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt4::W`](W) writer structure"]
impl crate::Writable for DT4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT4 to value 0"]
impl crate::Resettable for DT4_SPEC {
    const RESET_VALUE: u32 = 0;
}
