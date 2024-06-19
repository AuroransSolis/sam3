#[doc = "Register `CLRINT` writer"]
pub type W = crate::W<ClrintSpec>;
#[doc = "Field `DET_SUSPD` writer - Suspend Interrupt Clear"]
pub type DetSuspdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MICRO_SOF` writer - Micro Start Of Frame Interrupt Clear"]
pub type MicroSofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_SOF` writer - Start Of Frame Interrupt Clear"]
pub type IntSofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRESET` writer - End Of Reset Interrupt Clear"]
pub type EndresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE_UP` writer - Wake Up CPU Interrupt Clear"]
pub type WakeUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDOFRSM` writer - End Of Resume Interrupt Clear"]
pub type EndofrsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPSTR_RES` writer - Upstream Resume Interrupt Clear"]
pub type UpstrResW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Suspend Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn det_suspd(&mut self) -> DetSuspdW<ClrintSpec> {
        DetSuspdW::new(self, 1)
    }
    #[doc = "Bit 2 - Micro Start Of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn micro_sof(&mut self) -> MicroSofW<ClrintSpec> {
        MicroSofW::new(self, 2)
    }
    #[doc = "Bit 3 - Start Of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn int_sof(&mut self) -> IntSofW<ClrintSpec> {
        IntSofW::new(self, 3)
    }
    #[doc = "Bit 4 - End Of Reset Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn endreset(&mut self) -> EndresetW<ClrintSpec> {
        EndresetW::new(self, 4)
    }
    #[doc = "Bit 5 - Wake Up CPU Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wake_up(&mut self) -> WakeUpW<ClrintSpec> {
        WakeUpW::new(self, 5)
    }
    #[doc = "Bit 6 - End Of Resume Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn endofrsm(&mut self) -> EndofrsmW<ClrintSpec> {
        EndofrsmW::new(self, 6)
    }
    #[doc = "Bit 7 - Upstream Resume Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn upstr_res(&mut self) -> UpstrResW<ClrintSpec> {
        UpstrResW::new(self, 7)
    }
}
#[doc = "UDPHS Clear Interrupt Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrint::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrintSpec;
impl crate::RegisterSpec for ClrintSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clrint::W`](W) writer structure"]
impl crate::Writable for ClrintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
