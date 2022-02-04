<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <xsl:include href="include/common.xsl"/>

    <!-- Trigger selection on ??? is missing the correct enumerated values. -->
    <xsl:template match="/device/peripherals/peripheral[name='DACC']/registers/register[name='MR']/fields/field[name='TRGSEL']">
        <field>
            <name>TRGSEL</name>
            <description>Trigger Selection</description>
            <bitOffset>1</bitOffset>
            <bitWidth>3</bitWidth>
            <access>read-write</access>
            <enumeratedValues>
            <name>TRGSEL_A</name>
            <enumeratedValue>
                <name>EXTERNAL</name>
                <description>External trigger</description>
                <value>0x0</value>
            </enumeratedValue>
            <enumeratedValue>
                <name>TIOOTCC0</name>
                <description>TIO Output of the Timer Counter Channel 0</description>
                <value>0x1</value>
            </enumeratedValue>
            <enumeratedValue>
                <name>TIOOTCC1</name>
                <description>TIO Output of the Timer Counter Channel 1</description>
                <value>0x2</value>
            </enumeratedValue>
            <enumeratedValue>
                <name>TIOOTCC2</name>
                <description>TIO Output of the Timer Counter Channel 2</description>
                <value>0x3</value>
            </enumeratedValue>
            <enumeratedValue>
                <name>PWM0</name>
                <description>PWM Event Line 0</description>
                <value>0x4</value>
            </enumeratedValue>
            <enumeratedValue>
                <name>PWM1</name>
                <description>PWM Event Line 1</description>
                <value>0x5</value>
            </enumeratedValue>
            <enumeratedValue>
                <name>RESERVED</name>
                <description>Reserved</description>
                <value>0x6</value>
            </enumeratedValue>
            <enumeratedValue>
                <name>RESERVED</name>
                <description>Reserved</description>
                <value>0x7</value>
            </enumeratedValue>
            </enumeratedValues>
        </field>
    </xsl:template>
</xsl:stylesheet>
