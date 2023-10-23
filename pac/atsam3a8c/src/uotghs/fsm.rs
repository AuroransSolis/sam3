#[doc = "Register `FSM` reader"]
pub type R = crate::R<FSM_SPEC>;
#[doc = "Field `DRDSTATE` reader - Dual Role Device State"]
pub type DRDSTATE_R = crate::FieldReader<DRDSTATE_A>;
#[doc = "Dual Role Device State\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRDSTATE_A {
    #[doc = "0: This is the start state for A-devices (when the ID pin is 0)"]
    AIdlestate = 0,
    #[doc = "1: In this state, the A-device waits for the voltage on VBus to rise above the A-device VBus Valid threshold (4.4 V)."]
    AWaitVrise = 1,
    #[doc = "2: In this state, the A-device waits for the B-device to signal a connection."]
    AWaitBcon = 2,
    #[doc = "3: In this state, the A-device that operates in Host mode is operational."]
    AHost = 3,
    #[doc = "4: The A-device operating as a host is in the suspend mode."]
    ASuspend = 4,
    #[doc = "5: The A-device operates as a peripheral."]
    APeripheral = 5,
    #[doc = "6: In this state, the A-device waits for the voltage on VBus to drop below the A-device Session Valid threshold (1.4 V)."]
    AWaitVfall = 6,
    #[doc = "7: In this state, the A-device waits for recovery of the over-current condition that caused it to enter this state."]
    AVbusErr = 7,
    #[doc = "8: In this state, the A-device waits for the data USB line to discharge (100 us)."]
    AWaitDischarge = 8,
    #[doc = "9: This is the start state for B-device (when the ID pin is 1)."]
    BIdle = 9,
    #[doc = "10: In this state, the B-device acts as the peripheral."]
    BPeripheral = 10,
    #[doc = "11: In this state, the B-device is in suspend mode and waits until 3 ms before initiating the HNP protocol if requested."]
    BWaitBeginHnp = 11,
    #[doc = "12: In this state, the B-device waits for the data USB line to discharge (100 us) before becoming Host."]
    BWaitDischarge = 12,
    #[doc = "13: In this state, the B-device waits for the A-device to signal a connect before becoming B-Host."]
    BWaitAcon = 13,
    #[doc = "14: In this state, the B-device acts as the Host."]
    BHost = 14,
    #[doc = "15: In this state, the B-device attempts to start a session using the SRP protocol."]
    BSrpInit = 15,
}
impl From<DRDSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRDSTATE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DRDSTATE_A {
    type Ux = u8;
}
impl DRDSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRDSTATE_A {
        match self.bits {
            0 => DRDSTATE_A::AIdlestate,
            1 => DRDSTATE_A::AWaitVrise,
            2 => DRDSTATE_A::AWaitBcon,
            3 => DRDSTATE_A::AHost,
            4 => DRDSTATE_A::ASuspend,
            5 => DRDSTATE_A::APeripheral,
            6 => DRDSTATE_A::AWaitVfall,
            7 => DRDSTATE_A::AVbusErr,
            8 => DRDSTATE_A::AWaitDischarge,
            9 => DRDSTATE_A::BIdle,
            10 => DRDSTATE_A::BPeripheral,
            11 => DRDSTATE_A::BWaitBeginHnp,
            12 => DRDSTATE_A::BWaitDischarge,
            13 => DRDSTATE_A::BWaitAcon,
            14 => DRDSTATE_A::BHost,
            15 => DRDSTATE_A::BSrpInit,
            _ => unreachable!(),
        }
    }
    #[doc = "This is the start state for A-devices (when the ID pin is 0)"]
    #[inline(always)]
    pub fn is_a_idlestate(&self) -> bool {
        *self == DRDSTATE_A::AIdlestate
    }
    #[doc = "In this state, the A-device waits for the voltage on VBus to rise above the A-device VBus Valid threshold (4.4 V)."]
    #[inline(always)]
    pub fn is_a_wait_vrise(&self) -> bool {
        *self == DRDSTATE_A::AWaitVrise
    }
    #[doc = "In this state, the A-device waits for the B-device to signal a connection."]
    #[inline(always)]
    pub fn is_a_wait_bcon(&self) -> bool {
        *self == DRDSTATE_A::AWaitBcon
    }
    #[doc = "In this state, the A-device that operates in Host mode is operational."]
    #[inline(always)]
    pub fn is_a_host(&self) -> bool {
        *self == DRDSTATE_A::AHost
    }
    #[doc = "The A-device operating as a host is in the suspend mode."]
    #[inline(always)]
    pub fn is_a_suspend(&self) -> bool {
        *self == DRDSTATE_A::ASuspend
    }
    #[doc = "The A-device operates as a peripheral."]
    #[inline(always)]
    pub fn is_a_peripheral(&self) -> bool {
        *self == DRDSTATE_A::APeripheral
    }
    #[doc = "In this state, the A-device waits for the voltage on VBus to drop below the A-device Session Valid threshold (1.4 V)."]
    #[inline(always)]
    pub fn is_a_wait_vfall(&self) -> bool {
        *self == DRDSTATE_A::AWaitVfall
    }
    #[doc = "In this state, the A-device waits for recovery of the over-current condition that caused it to enter this state."]
    #[inline(always)]
    pub fn is_a_vbus_err(&self) -> bool {
        *self == DRDSTATE_A::AVbusErr
    }
    #[doc = "In this state, the A-device waits for the data USB line to discharge (100 us)."]
    #[inline(always)]
    pub fn is_a_wait_discharge(&self) -> bool {
        *self == DRDSTATE_A::AWaitDischarge
    }
    #[doc = "This is the start state for B-device (when the ID pin is 1)."]
    #[inline(always)]
    pub fn is_b_idle(&self) -> bool {
        *self == DRDSTATE_A::BIdle
    }
    #[doc = "In this state, the B-device acts as the peripheral."]
    #[inline(always)]
    pub fn is_b_peripheral(&self) -> bool {
        *self == DRDSTATE_A::BPeripheral
    }
    #[doc = "In this state, the B-device is in suspend mode and waits until 3 ms before initiating the HNP protocol if requested."]
    #[inline(always)]
    pub fn is_b_wait_begin_hnp(&self) -> bool {
        *self == DRDSTATE_A::BWaitBeginHnp
    }
    #[doc = "In this state, the B-device waits for the data USB line to discharge (100 us) before becoming Host."]
    #[inline(always)]
    pub fn is_b_wait_discharge(&self) -> bool {
        *self == DRDSTATE_A::BWaitDischarge
    }
    #[doc = "In this state, the B-device waits for the A-device to signal a connect before becoming B-Host."]
    #[inline(always)]
    pub fn is_b_wait_acon(&self) -> bool {
        *self == DRDSTATE_A::BWaitAcon
    }
    #[doc = "In this state, the B-device acts as the Host."]
    #[inline(always)]
    pub fn is_b_host(&self) -> bool {
        *self == DRDSTATE_A::BHost
    }
    #[doc = "In this state, the B-device attempts to start a session using the SRP protocol."]
    #[inline(always)]
    pub fn is_b_srp_init(&self) -> bool {
        *self == DRDSTATE_A::BSrpInit
    }
}
impl R {
    #[doc = "Bits 0:3 - Dual Role Device State"]
    #[inline(always)]
    pub fn drdstate(&self) -> DRDSTATE_R {
        DRDSTATE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "General Finite State Machine Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm::R`](R) reader structure"]
impl crate::Readable for FSM_SPEC {}
#[doc = "`reset()` method sets FSM to value 0x09"]
impl crate::Resettable for FSM_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
