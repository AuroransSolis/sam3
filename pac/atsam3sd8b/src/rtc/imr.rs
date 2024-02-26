#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `ACK` reader - Acknowledge Update Interrupt Mask"]
pub type AckR = crate::BitReader;
#[doc = "Field `ALR` reader - Alarm Interrupt Mask"]
pub type AlrR = crate::BitReader;
#[doc = "Field `SEC` reader - Second Event Interrupt Mask"]
pub type SecR = crate::BitReader;
#[doc = "Field `TIM` reader - Time Event Interrupt Mask"]
pub type TimR = crate::BitReader;
#[doc = "Field `CAL` reader - Calendar Event Interrupt Mask"]
pub type CalR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Acknowledge Update Interrupt Mask"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn alr(&self) -> AlrR {
        AlrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Second Event Interrupt Mask"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Event Interrupt Mask"]
    #[inline(always)]
    pub fn tim(&self) -> TimR {
        TimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Calendar Event Interrupt Mask"]
    #[inline(always)]
    pub fn cal(&self) -> CalR {
        CalR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
