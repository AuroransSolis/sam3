#[doc = "Register `LPR` reader"]
pub type R = crate::R<LPR_SPEC>;
#[doc = "Register `LPR` writer"]
pub type W = crate::W<LPR_SPEC>;
#[doc = "Field `LPCB` reader - Low-power Configuration Bits"]
pub type LPCB_R = crate::FieldReader<LPCB_A>;
#[doc = "Low-power Configuration Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPCB_A {
    #[doc = "0: Low Power Feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    Disabled = 0,
    #[doc = "1: The SDRAM Controller issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self Refresh Mode when accessed and enters it after the access."]
    SelfRefresh = 1,
    #[doc = "2: The SDRAM Controller issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down Mode when accessed and enters it after the access."]
    PowerDown = 2,
    #[doc = "3: The SDRAM Controller issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    DeepPowerDown = 3,
}
impl From<LPCB_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPCB_A {
    type Ux = u8;
}
impl LPCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCB_A {
        match self.bits {
            0 => LPCB_A::Disabled,
            1 => LPCB_A::SelfRefresh,
            2 => LPCB_A::PowerDown,
            3 => LPCB_A::DeepPowerDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Low Power Feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPCB_A::Disabled
    }
    #[doc = "The SDRAM Controller issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self Refresh Mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn is_self_refresh(&self) -> bool {
        *self == LPCB_A::SelfRefresh
    }
    #[doc = "The SDRAM Controller issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down Mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == LPCB_A::PowerDown
    }
    #[doc = "The SDRAM Controller issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    #[inline(always)]
    pub fn is_deep_power_down(&self) -> bool {
        *self == LPCB_A::DeepPowerDown
    }
}
#[doc = "Field `LPCB` writer - Low-power Configuration Bits"]
pub type LPCB_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, LPCB_A>;
impl<'a, REG, const O: u8> LPCB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Power Feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LPCB_A::Disabled)
    }
    #[doc = "The SDRAM Controller issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self Refresh Mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn self_refresh(self) -> &'a mut crate::W<REG> {
        self.variant(LPCB_A::SelfRefresh)
    }
    #[doc = "The SDRAM Controller issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down Mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(LPCB_A::PowerDown)
    }
    #[doc = "The SDRAM Controller issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    #[inline(always)]
    pub fn deep_power_down(self) -> &'a mut crate::W<REG> {
        self.variant(LPCB_A::DeepPowerDown)
    }
}
#[doc = "Field `PASR` reader - Partial Array Self-refresh (only for low-power SDRAM)"]
pub type PASR_R = crate::FieldReader;
#[doc = "Field `PASR` writer - Partial Array Self-refresh (only for low-power SDRAM)"]
pub type PASR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TCSR` reader - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
pub type TCSR_R = crate::FieldReader;
#[doc = "Field `TCSR` writer - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
pub type TCSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DS` reader - Drive Strength (only for low-power SDRAM)"]
pub type DS_R = crate::FieldReader;
#[doc = "Field `DS` writer - Drive Strength (only for low-power SDRAM)"]
pub type DS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TIMEOUT` reader - Time to define when low-power mode is enable"]
pub type TIMEOUT_R = crate::FieldReader<TIMEOUT_A>;
#[doc = "Time to define when low-power mode is enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: The SDRAM controller activates the SDRAM low-power mode immediately after the end of the last transfer."]
    LpLastXfer = 0,
    #[doc = "1: The SDRAM controller activates the SDRAM low-power mode 64 clock cycles after the end of the last transfer."]
    LpLastXfer64 = 1,
    #[doc = "2: The SDRAM controller activates the SDRAM low-power mode 128 clock cycles after the end of the last transfer."]
    LpLastXfer128 = 2,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMEOUT_A {
    type Ux = u8;
}
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMEOUT_A> {
        match self.bits {
            0 => Some(TIMEOUT_A::LpLastXfer),
            1 => Some(TIMEOUT_A::LpLastXfer64),
            2 => Some(TIMEOUT_A::LpLastXfer128),
            _ => None,
        }
    }
    #[doc = "The SDRAM controller activates the SDRAM low-power mode immediately after the end of the last transfer."]
    #[inline(always)]
    pub fn is_lp_last_xfer(&self) -> bool {
        *self == TIMEOUT_A::LpLastXfer
    }
    #[doc = "The SDRAM controller activates the SDRAM low-power mode 64 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn is_lp_last_xfer_64(&self) -> bool {
        *self == TIMEOUT_A::LpLastXfer64
    }
    #[doc = "The SDRAM controller activates the SDRAM low-power mode 128 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn is_lp_last_xfer_128(&self) -> bool {
        *self == TIMEOUT_A::LpLastXfer128
    }
}
#[doc = "Field `TIMEOUT` writer - Time to define when low-power mode is enable"]
pub type TIMEOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TIMEOUT_A>;
impl<'a, REG, const O: u8> TIMEOUT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The SDRAM controller activates the SDRAM low-power mode immediately after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_A::LpLastXfer)
    }
    #[doc = "The SDRAM controller activates the SDRAM low-power mode 64 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer_64(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_A::LpLastXfer64)
    }
    #[doc = "The SDRAM controller activates the SDRAM low-power mode 128 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer_128(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_A::LpLastXfer128)
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline(always)]
    pub fn lpcb(&self) -> LPCB_R {
        LPCB_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn pasr(&self) -> PASR_R {
        PASR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn tcsr(&self) -> TCSR_R {
        TCSR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Time to define when low-power mode is enable"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline(always)]
    #[must_use]
    pub fn lpcb(&mut self) -> LPCB_W<LPR_SPEC, 0> {
        LPCB_W::new(self)
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline(always)]
    #[must_use]
    pub fn pasr(&mut self) -> PASR_W<LPR_SPEC, 4> {
        PASR_W::new(self)
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline(always)]
    #[must_use]
    pub fn tcsr(&mut self) -> TCSR_W<LPR_SPEC, 8> {
        TCSR_W::new(self)
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<LPR_SPEC, 10> {
        DS_W::new(self)
    }
    #[doc = "Bits 12:13 - Time to define when low-power mode is enable"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<LPR_SPEC, 12> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SDRAMC Low Power Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPR_SPEC;
impl crate::RegisterSpec for LPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpr::R`](R) reader structure"]
impl crate::Readable for LPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpr::W`](W) writer structure"]
impl crate::Writable for LPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPR to value 0"]
impl crate::Resettable for LPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
